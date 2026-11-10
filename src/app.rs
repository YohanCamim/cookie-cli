use crate::data::{ACHIEVEMENTS, GARDEN_SEEDS, HEAVENLY_UPGRADES, NUM_BUILDINGS, STOCK_SYMBOLS};
use crate::engine::{Action, GameState};
use crate::save::{export_base64, import_base64, save_to_path};
use crate::ui::anim::AnimState;
use crate::ui::{render, CenterTab, ModalState, UiViewState};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const LOGIC_TPS: f64 = 30.0;
const LOGIC_DT: f64 = 1.0 / LOGIC_TPS;
const AUTOSAVE_INTERVAL_SECS: f64 = 60.0;
const MIN_CLICK_INTERVAL: Duration = Duration::from_millis(50); // 20 clicks/sec cap

pub struct AppConfig {
    pub save_path: PathBuf,
    pub fps: u32,
    pub ascii_only: bool,
    pub no_color: bool,
}

pub struct App {
    pub state: GameState,
    pub anim: AnimState,
    pub view: UiViewState,
    pub config: AppConfig,
    pub should_quit: bool,
    last_click_instant: Option<Instant>,
    autosave_accum: f64,
}

impl App {
    pub fn new(state: GameState, config: AppConfig) -> Self {
        let mut anim = AnimState::new();
        anim.displayed_cookies = state.cookies;
        let view = UiViewState {
            ascii_only: config.ascii_only,
            no_color: config.no_color,
            ..Default::default()
        };
        Self {
            state,
            anim,
            view,
            config,
            should_quit: false,
            last_click_instant: None,
            autosave_accum: 0.0,
        }
    }

    /// Runs the main 30 TPS fixed-step logic & configurable FPS render loop.
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), String> {
        let fps = self.config.fps.clamp(10, 120);
        let frame_duration = Duration::from_secs_f64(1.0 / (fps as f64));
        let mut last_instant = Instant::now();
        let mut logic_accum = 0.0_f64;

        while !self.should_quit {
            let now = Instant::now();
            let elapsed = now.duration_since(last_instant).as_secs_f64().min(0.25);
            last_instant = now;

            logic_accum += elapsed;
            while logic_accum >= LOGIC_DT {
                self.state.tick(LOGIC_DT);
                logic_accum -= LOGIC_DT;
            }

            self.drain_achievement_toasts();
            self.anim.update(elapsed as f32, self.state.cookies);

            self.autosave_accum += elapsed;
            if self.autosave_accum >= AUTOSAVE_INTERVAL_SECS {
                self.autosave_accum = 0.0;
                let _ = save_to_path(&self.state, &self.config.save_path);
            }

            terminal
                .draw(|frame| render(frame, &self.state, &self.anim, &self.view))
                .map_err(|e| format!("Terminal draw error: {e}"))?;

            let poll_timeout = frame_duration.saturating_sub(Instant::now().duration_since(now));
            if event::poll(poll_timeout).map_err(|e| format!("Event poll error: {e}"))? {
                if let Event::Key(key) =
                    event::read().map_err(|e| format!("Event read error: {e}"))?
                {
                    self.handle_key_event(key, Instant::now());
                }
            }
        }

        let _ = save_to_path(&self.state, &self.config.save_path);
        Ok(())
    }

    fn drain_achievement_toasts(&mut self) {
        while let Some(ach_id) = self.state.newly_unlocked_achievements.pop() {
            if let Some(ach) = ACHIEVEMENTS.get(ach_id) {
                self.anim
                    .push_toast(ach.name.to_string(), ach.description.to_string());
            }
        }
    }

    /// Processes a single keyboard event, enforcing `KeyEventKind::Press` and the 20 CPS click cap.
    pub fn handle_key_event(&mut self, key: KeyEvent, now: Instant) {
        // Strictly ignore Release and Repeat events to prevent cross-platform key-repeat flood
        if key.kind != KeyEventKind::Press {
            return;
        }

        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        // Handle active modal first
        if self.view.modal != ModalState::None {
            self.handle_modal_key(key);
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.try_click_cookie(now);
            }
            KeyCode::Tab => {
                self.view.tab = self.view.tab.next();
            }
            KeyCode::BackTab => {
                self.view.tab = self.view.tab.prev();
            }
            KeyCode::Char('1') | KeyCode::F(1) => self.view.tab = CenterTab::Buildings,
            KeyCode::Char('2') | KeyCode::F(2) => self.view.tab = CenterTab::Upgrades,
            KeyCode::Char('3') | KeyCode::F(3) => self.view.tab = CenterTab::Special,
            KeyCode::Char('4') | KeyCode::F(4) => self.view.tab = CenterTab::Prestige,
            KeyCode::Char('5') | KeyCode::F(5) => self.view.tab = CenterTab::Achievements,
            KeyCode::Char('6') | KeyCode::F(6) => self.view.tab = CenterTab::Help,

            KeyCode::Up => match self.view.tab {
                CenterTab::Upgrades => {
                    self.view.selected_upgrade_idx =
                        self.view.selected_upgrade_idx.saturating_sub(1);
                }
                CenterTab::Prestige => {
                    self.view.selected_heavenly_idx =
                        self.view.selected_heavenly_idx.saturating_sub(1);
                }
                CenterTab::Special => {
                    self.view.selected_stock_idx = self.view.selected_stock_idx.saturating_sub(1);
                }
                _ => {
                    self.view.selected_building = self.view.selected_building.saturating_sub(1);
                }
            },
            KeyCode::Down => match self.view.tab {
                CenterTab::Upgrades => {
                    let max_idx = self.state.available_upgrades().len().saturating_sub(1);
                    self.view.selected_upgrade_idx =
                        (self.view.selected_upgrade_idx + 1).min(max_idx);
                }
                CenterTab::Prestige => {
                    let max_idx = HEAVENLY_UPGRADES.len().saturating_sub(1);
                    self.view.selected_heavenly_idx =
                        (self.view.selected_heavenly_idx + 1).min(max_idx);
                }
                CenterTab::Special => {
                    let max_idx = STOCK_SYMBOLS.len().saturating_sub(1);
                    self.view.selected_stock_idx = (self.view.selected_stock_idx + 1).min(max_idx);
                }
                _ => {
                    self.view.selected_building =
                        (self.view.selected_building + 1).min(NUM_BUILDINGS - 1);
                }
            },
            KeyCode::Right | KeyCode::Char('y') | KeyCode::Char('Y') => {
                self.state
                    .apply(Action::BuyOrSellBuilding(self.view.selected_building));
                self.drain_achievement_toasts();
            }
            KeyCode::Char('b') | KeyCode::Char('B') => {
                self.state.apply(Action::ToggleStoreMode);
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                self.state.apply(Action::CycleBuyAmount);
            }
            KeyCode::Char('u') | KeyCode::Char('U') => {
                let avail = self.state.available_upgrades();
                if !avail.is_empty() {
                    let idx = if self.view.tab == CenterTab::Upgrades {
                        self.view.selected_upgrade_idx.min(avail.len() - 1)
                    } else {
                        0
                    };
                    self.state.apply(Action::BuyUpgrade(avail[idx]));
                    self.drain_achievement_toasts();
                }
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.state.apply(Action::ClickShimmer);
                self.drain_achievement_toasts();
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.state.apply(Action::PopAllWrinklers);
                self.drain_achievement_toasts();
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                self.state
                    .apply(Action::LevelUpBuilding(self.view.selected_building));
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.state.apply(Action::TrainDragon);
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.state.apply(Action::CycleDragonAura);
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                self.state.apply(Action::CycleSeason);
            }
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                self.state.apply(Action::CastSpell(0));
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.state.apply(Action::CastSpell(1));
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                self.state.apply(Action::CastSpell(2));
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                let next_seed = (self.state.garden_selected_seed + 1) % GARDEN_SEEDS.len();
                self.state.apply(Action::GardenSelectSeed(next_seed));
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                let (r, c) = self.view.garden_cursor;
                self.state.apply(Action::GardenInteractPlot(r, c));
                let next_c = (c + 1) % 4;
                let next_r = if next_c == 0 { (r + 1) % 4 } else { r };
                self.view.garden_cursor = (next_r, next_c);
            }
            KeyCode::Char('k') | KeyCode::Char('K') => {
                self.state
                    .apply(Action::BuyStock(self.view.selected_stock_idx));
            }
            KeyCode::Char('j') | KeyCode::Char('J') => {
                self.state
                    .apply(Action::SellStock(self.view.selected_stock_idx));
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.state.apply(Action::CyclePantheonSpirit);
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.view.modal = ModalState::AscendConfirm;
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.state
                    .apply(Action::BuyHeavenlyUpgrade(self.view.selected_heavenly_idx));
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                match save_to_path(&self.state, &self.config.save_path) {
                    Ok(()) => {
                        self.state.last_event_message = "Game saved to disk.".to_string();
                    }
                    Err(e) => {
                        self.state.last_event_message = format!("Save error: {e}");
                    }
                }
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                if let Ok(code) = export_base64(&self.state) {
                    self.view.modal = ModalState::ExportSave(code);
                }
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.view.modal = ModalState::ImportSave(String::new());
            }
            _ => {}
        }
    }

    fn try_click_cookie(&mut self, now: Instant) {
        if let Some(last) = self.last_click_instant {
            if now.duration_since(last) < MIN_CLICK_INTERVAL {
                return;
            }
        }
        self.last_click_instant = Some(now);
        self.state.apply(Action::ClickCookie);
        self.anim
            .on_cookie_click(self.state.last_click_gain, self.view.ascii_only);
        self.drain_achievement_toasts();
    }

    fn handle_modal_key(&mut self, key: KeyEvent) {
        match &mut self.view.modal {
            ModalState::None => {}
            ModalState::AscendConfirm => match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                    self.state.apply(Action::Ascend);
                    self.anim.displayed_cookies = self.state.cookies;
                    self.view.modal = ModalState::None;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.view.modal = ModalState::None;
                }
                _ => {}
            },
            ModalState::ResetConfirm => match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.state = GameState::new();
                    self.anim = AnimState::new();
                    self.view.modal = ModalState::None;
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.view.modal = ModalState::None;
                }
                _ => {}
            },
            ModalState::ExportSave(_) => {
                if matches!(key.code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q')) {
                    self.view.modal = ModalState::None;
                }
            }
            ModalState::ImportSave(buf) => match key.code {
                KeyCode::Esc => {
                    self.view.modal = ModalState::None;
                }
                KeyCode::Backspace => {
                    buf.pop();
                }
                KeyCode::Enter => {
                    let input = buf.clone();
                    match import_base64(&input) {
                        Ok(new_state) => {
                            self.state = new_state;
                            self.anim.displayed_cookies = self.state.cookies;
                            self.view.modal = ModalState::None;
                        }
                        Err(e) => {
                            self.state.last_event_message = format!("Import failed: {e}");
                            self.view.modal = ModalState::None;
                        }
                    }
                }
                KeyCode::Char(c) => {
                    buf.push(c);
                }
                _ => {}
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyEventState;

    fn make_key(code: KeyCode, kind: KeyEventKind) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn test_anti_autoclick_and_key_release_filtering() {
        let state = GameState::with_seed(1);
        let config = AppConfig {
            save_path: PathBuf::from("test_unused.json"),
            fps: 60,
            ascii_only: false,
            no_color: false,
        };
        let mut app = App::new(state, config);
        let t0 = Instant::now();

        // KeyRelease and KeyRepeat must be ignored
        app.handle_key_event(make_key(KeyCode::Char(' '), KeyEventKind::Release), t0);
        app.handle_key_event(make_key(KeyCode::Char(' '), KeyEventKind::Repeat), t0);
        assert_eq!(app.state.total_clicks, 0);

        // First press succeeds
        app.handle_key_event(make_key(KeyCode::Char(' '), KeyEventKind::Press), t0);
        assert_eq!(app.state.total_clicks, 1);

        // Press 10ms later is rate-limited (< 50ms)
        app.handle_key_event(
            make_key(KeyCode::Char(' '), KeyEventKind::Press),
            t0 + Duration::from_millis(10),
        );
        assert_eq!(app.state.total_clicks, 1);

        // Press 55ms later succeeds
        app.handle_key_event(
            make_key(KeyCode::Char(' '), KeyEventKind::Press),
            t0 + Duration::from_millis(55),
        );
        assert_eq!(app.state.total_clicks, 2);
    }
}
