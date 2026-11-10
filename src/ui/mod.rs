pub mod anim;

use crate::data::{
    ACHIEVEMENTS, BUILDINGS, GARDEN_SEEDS, GRIMOIRE_SPELLS, HEAVENLY_UPGRADES, NUM_BUILDINGS,
    PANTHEON_SPIRITS, STOCK_SYMBOLS, UPGRADES,
};
use crate::engine::{GameState, StoreMode};
use crate::format::{format_duration, format_long, format_number, format_rate};
use crate::ui::anim::AnimState;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap};
use ratatui::Frame;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CenterTab {
    #[default]
    Buildings,
    Upgrades,
    Special,
    Prestige,
    Achievements,
    Help,
}

impl CenterTab {
    pub const ALL: [CenterTab; 6] = [
        CenterTab::Buildings,
        CenterTab::Upgrades,
        CenterTab::Special,
        CenterTab::Prestige,
        CenterTab::Achievements,
        CenterTab::Help,
    ];

    pub fn next(self) -> Self {
        let idx = Self::ALL.iter().position(|&t| t == self).unwrap_or(0);
        Self::ALL[(idx + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        let idx = Self::ALL.iter().position(|&t| t == self).unwrap_or(0);
        Self::ALL[(idx + Self::ALL.len() - 1) % Self::ALL.len()]
    }

    pub fn title(self) -> &'static str {
        match self {
            CenterTab::Buildings => "1:Buildings",
            CenterTab::Upgrades => "2:Upgrades",
            CenterTab::Special => "3:Special",
            CenterTab::Prestige => "4:Prestige",
            CenterTab::Achievements => "5:Achiev",
            CenterTab::Help => "6:Help",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ModalState {
    #[default]
    None,
    AscendConfirm,
    ResetConfirm,
    ExportSave(String),
    ImportSave(String),
}

#[derive(Debug, Clone)]
pub struct UiViewState {
    pub tab: CenterTab,
    pub selected_building: usize,
    pub selected_upgrade_idx: usize,
    pub selected_heavenly_idx: usize,
    pub garden_cursor: (usize, usize),
    pub selected_stock_idx: usize,
    pub modal: ModalState,
    pub ascii_only: bool,
    pub no_color: bool,
}

impl Default for UiViewState {
    fn default() -> Self {
        Self {
            tab: CenterTab::Buildings,
            selected_building: 0,
            selected_upgrade_idx: 0,
            selected_heavenly_idx: 0,
            garden_cursor: (0, 0),
            selected_stock_idx: 0,
            modal: ModalState::None,
            ascii_only: false,
            no_color: false,
        }
    }
}

impl UiViewState {
    pub fn fg(&self, color: Color) -> Style {
        if self.no_color {
            Style::default()
        } else {
            Style::default().fg(color)
        }
    }

    pub fn bold_fg(&self, color: Color) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        }
    }

    pub fn highlight_style(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    }
}

const BIG_COOKIE_NORMAL: &[&str] = &[
    r"      .---.      ",
    r"   .-'  o  '-.   ",
    r"  /  o     o  \  ",
    r" | o    o    o | ",
    r" |    o     o  | ",
    r"  \  o   o    /  ",
    r"   '-.  o  .-'   ",
    r"      '---'      ",
];

const BIG_COOKIE_PULSE: &[&str] = &[
    r"     .-----.     ",
    r"  .-' o   o '-.  ",
    r" /  o   o   o  \ ",
    r"|  o   (o)   o  |",
    r"|   o   o   o   |",
    r" \  o   o   o  / ",
    r"  '-. o   o .-'  ",
    r"     '-----'     ",
];

/// Renders the entire TUI frame.
pub fn render(frame: &mut Frame, state: &GameState, anim: &AnimState, view: &UiViewState) {
    let size = frame.area();

    if size.width < 60 || size.height < 18 {
        let msg = format!(
            "Terminal too small ({}x{}).\nPlease resize to at least 60x18.",
            size.width, size.height
        );
        let block = Paragraph::new(msg)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" Cookie-CLI "));
        frame.render_widget(block, size);
        return;
    }

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(28),
            Constraint::Percentage(42),
            Constraint::Percentage(30),
        ])
        .split(size);

    render_left_bakery(frame, columns[0], state, anim, view);
    render_center_panel(frame, columns[1], state, anim, view);
    render_right_store(frame, columns[2], state, view);

    if view.modal != ModalState::None {
        render_modal(frame, size, state, view);
    }
}

fn render_left_bakery(
    frame: &mut Frame,
    area: Rect,
    state: &GameState,
    anim: &AnimState,
    view: &UiViewState,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {}'s Bakery ", state.bakery_name))
        .border_style(view.fg(Color::Yellow));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 6 || inner.width < 10 {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(9),
            Constraint::Length(4),
            Constraint::Length(3),
        ])
        .split(inner);

    // 1. Cookie counter + CPS/CPC
    let leech_pct = (state.wrinkler_leech_fraction() * 100.0).round() as u32;
    let cps_str = if leech_pct > 0 {
        format!(
            "per second: {} (withered {}%)",
            format_rate(state.effective_cps()),
            leech_pct
        )
    } else {
        format!("per second: {}", format_rate(state.effective_cps()))
    };

    let header_lines = vec![
        Line::from(Span::styled(
            format!("{} cookies", format_number(anim.displayed_cookies)),
            view.bold_fg(Color::Yellow),
        )),
        Line::from(Span::styled(cps_str, view.fg(Color::Cyan))),
        Line::from(Span::styled(
            format!("per click: {}", format_rate(state.cookies_per_click())),
            view.fg(Color::Green),
        )),
    ];
    frame.render_widget(
        Paragraph::new(header_lines).alignment(Alignment::Center),
        chunks[0],
    );

    // 2. Big Cookie canvas + particles + floating text + shimmer alert
    let cookie_art = if anim.cookie_pulse > 0.25 {
        BIG_COOKIE_PULSE
    } else {
        BIG_COOKIE_NORMAL
    };

    let mut canvas_lines: Vec<Line> = Vec::new();
    if let Some(shimmer) = &state.active_shimmer {
        let blink = anim.shimmer_phase.sin() > 0.0;
        let label = if shimmer.is_wrath {
            format!(
                "!! [G] WRATH COOKIE ({:.1}s) !!",
                shimmer.time_left.max(0.0)
            )
        } else {
            format!(
                "** [G] GOLDEN COOKIE ({:.1}s) **",
                shimmer.time_left.max(0.0)
            )
        };
        let color = if shimmer.is_wrath {
            Color::LightRed
        } else if blink {
            Color::LightYellow
        } else {
            Color::Yellow
        };
        canvas_lines.push(Line::from(Span::styled(label, view.bold_fg(color))));
    } else {
        canvas_lines.push(Line::from(Span::styled(
            "[SPACE / ENTER] Bake Cookie",
            view.fg(Color::DarkGray),
        )));
    }

    for (idx, row_str) in cookie_art.iter().enumerate() {
        let mut line_str = (*row_str).to_string();
        // Overlay floating text near top rows
        if idx == 0 {
            if let Some(ft) = anim.floating_texts.last() {
                line_str = format!("  {}  {}", ft.text, row_str.trim());
            }
        } else if idx == cookie_art.len() - 1 && !anim.particles.is_empty() {
            let p = &anim.particles[anim.particles.len() - 1];
            line_str = format!(" {} {} {} ", p.glyph, row_str.trim(), p.glyph);
        }
        let cookie_color = if anim.cookie_pulse > 0.25 {
            Color::LightYellow
        } else {
            Color::Yellow
        };
        canvas_lines.push(Line::from(Span::styled(
            line_str,
            view.bold_fg(cookie_color),
        )));
    }

    frame.render_widget(
        Paragraph::new(canvas_lines).alignment(Alignment::Center),
        chunks[1],
    );

    // 3. Buffs & Wrinklers status
    let mut status_lines = Vec::new();
    if state.active_buffs.is_empty() {
        status_lines.push(Line::from(Span::styled(
            format!(
                "Lumps: {} | Dragon Lv.{}",
                state.sugar_lumps, state.dragon_level
            ),
            view.fg(Color::Magenta),
        )));
    } else {
        for b in state.active_buffs.iter().take(2) {
            status_lines.push(Line::from(Span::styled(
                format!("{} ({:.0}s)", b.kind.name(), b.time_left),
                view.bold_fg(Color::LightGreen),
            )));
        }
    }

    let wrinklers_cnt = state.active_wrinkler_count();
    if wrinklers_cnt > 0 {
        status_lines.push(Line::from(Span::styled(
            format!("Wrinklers: {}/10 ([W] Pop All)", wrinklers_cnt),
            view.bold_fg(Color::Red),
        )));
    } else {
        status_lines.push(Line::from(Span::styled(
            format!("Grandmas: {}", state.grandmapocalypse_status_name()),
            view.fg(Color::Gray),
        )));
    }
    frame.render_widget(
        Paragraph::new(status_lines).alignment(Alignment::Center),
        chunks[2],
    );

    // 4. Animated Milk wave at bottom
    let wave_chars: &[char] = if view.ascii_only {
        &['~', '-', '~', '_']
    } else {
        &['≈', '~', '∼', '≈']
    };
    let width = chunks[3].width as usize;
    let phase_idx = (anim.milk_phase * 2.0) as usize;
    let wave_line: String = (0..width)
        .map(|x| wave_chars[(x + phase_idx) % wave_chars.len()])
        .collect();
    let milk_pct = (state.milk_progress() * 100.0).round() as u32;
    let milk_info = format!("{} ({}%)", state.milk_name(), milk_pct);
    let milk_para = Paragraph::new(vec![
        Line::from(Span::styled(wave_line, view.fg(Color::White))),
        Line::from(Span::styled(milk_info, view.bold_fg(Color::Cyan))),
    ])
    .alignment(Alignment::Center);
    frame.render_widget(milk_para, chunks[3]);
}

fn render_center_panel(
    frame: &mut Frame,
    area: Rect,
    state: &GameState,
    anim: &AnimState,
    view: &UiViewState,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    // 1. News Ticker
    let ticker_text = format!(
        "NEWS: {}   |   {}",
        state.current_news, state.last_event_message
    );
    let chars: Vec<char> = ticker_text.chars().collect();
    let max_w = chunks[0].width.saturating_sub(4) as usize;
    let visible_news: String = if chars.len() <= max_w || max_w == 0 {
        ticker_text
    } else {
        let start = anim.ticker_offset % chars.len();
        (0..max_w)
            .map(|i| chars[(start + i) % chars.len()])
            .collect()
    };
    let news_block = Paragraph::new(Line::from(Span::styled(
        visible_news,
        view.fg(Color::LightCyan),
    )))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" News Ticker "),
    );
    frame.render_widget(news_block, chunks[0]);

    // 2. Navigation Tabs
    let titles: Vec<Line> = CenterTab::ALL
        .iter()
        .map(|t| Line::from(t.title()))
        .collect();
    let selected_tab_idx = CenterTab::ALL
        .iter()
        .position(|&t| t == view.tab)
        .unwrap_or(0);
    let tabs = Tabs::new(titles)
        .select(selected_tab_idx)
        .highlight_style(view.highlight_style())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" [Tab / F1-F6] Section "),
        );
    frame.render_widget(tabs, chunks[1]);

    // 3. Active Center Tab Body
    match view.tab {
        CenterTab::Buildings => render_tab_buildings(frame, chunks[2], state, view),
        CenterTab::Upgrades => render_tab_upgrades(frame, chunks[2], state, view),
        CenterTab::Special => render_tab_special(frame, chunks[2], state, view),
        CenterTab::Prestige => render_tab_prestige(frame, chunks[2], state, view),
        CenterTab::Achievements => render_tab_achievements(frame, chunks[2], state, view),
        CenterTab::Help => render_tab_help(frame, chunks[2], view),
    }

    // 4. Footer / Toast notification bar
    let footer_line = if let Some(toast) = anim.toasts.last() {
        Line::from(Span::styled(
            format!("★ ACHIEVEMENT: {} - {} ★", toast.title, toast.description),
            view.bold_fg(Color::Yellow),
        ))
    } else {
        Line::from(Span::styled(
            "[SPACE] Click  [Up/Down] Select  [Enter] Buy/Sell  [U] Upgrade  [G] Golden  [S] Save  [Q] Quit",
            view.fg(Color::Gray),
        ))
    };
    let footer = Paragraph::new(footer_line)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[3]);
}

fn render_tab_buildings(frame: &mut Frame, area: Rect, state: &GameState, view: &UiViewState) {
    let raw_total = state.raw_cps().max(1e-9);
    let mut items = Vec::with_capacity(NUM_BUILDINGS);

    for (id, def) in BUILDINGS.iter().enumerate() {
        let count = state.buildings[id];
        let lvl = state.building_levels[id];
        let unit_cps = state.single_building_cps(id) * state.global_cps_multiplier();
        let total_b_cps = unit_cps * (count as f64);
        let pct = if state.raw_cps() > 0.0 {
            (total_b_cps / raw_total) * 100.0
        } else {
            0.0
        };
        let icon = if view.ascii_only {
            def.ascii_icon
        } else {
            def.icon
        };
        let line = format!(
            "{} {:<18} | Owned: {:>4} | Lv.{:<2} | CpS: {:>9} ({:>4.1}%)",
            icon,
            def.name,
            count,
            lvl,
            format_rate(total_b_cps),
            pct
        );
        let style = if id == view.selected_building {
            view.highlight_style()
        } else if count > 0 {
            view.fg(Color::White)
        } else {
            view.fg(Color::DarkGray)
        };
        items.push(ListItem::new(Line::from(Span::styled(line, style))));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Bakery Production ([L] Spend Sugar Lump to Level Up Selected) "),
    );
    frame.render_widget(list, area);
}

fn render_tab_upgrades(frame: &mut Frame, area: Rect, state: &GameState, view: &UiViewState) {
    let avail = state.available_upgrades();
    let mut items = Vec::new();

    if avail.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "No upgrades currently unlocked. Bake more cookies or buy more buildings!",
            view.fg(Color::DarkGray),
        ))));
    } else {
        let sel = view.selected_upgrade_idx.min(avail.len().saturating_sub(1));
        for (idx, &upg_id) in avail.iter().enumerate() {
            let upg = &UPGRADES[upg_id];
            let affordable = state.cookies >= upg.cost;
            let status = if affordable { "[READY]" } else { "[LOCKED]" };
            let text = format!(
                "{} {} ({} cookies) - {}",
                status,
                upg.name,
                format_number(upg.cost),
                upg.description
            );
            let style = if idx == sel {
                view.highlight_style()
            } else if affordable {
                view.bold_fg(Color::Green)
            } else {
                view.fg(Color::Gray)
            };
            items.push(ListItem::new(Line::from(Span::styled(text, style))));
        }
    }

    let title = format!(
        " Upgrades: {} Available | {}/{} Purchased ([U] Buy Selected) ",
        avail.len(),
        state.purchased_upgrades_count(),
        UPGRADES.len()
    );
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));
    frame.render_widget(list, area);
}

fn render_tab_special(frame: &mut Frame, area: Rect, state: &GameState, view: &UiViewState) {
    let mut lines = Vec::new();

    // Dragon & Seasons
    let dragon_cost = 10_000.0 * 10_f64.powi(state.dragon_level as i32);
    lines.push(Line::from(Span::styled(
        format!(
            "Dragon Krumblor: Stage {}/4 | Aura: {} ({})",
            state.dragon_level,
            state.dragon_aura.name(),
            state.dragon_aura.description()
        ),
        view.bold_fg(Color::LightRed),
    )));
    if state.dragon_level < 4 {
        lines.push(Line::from(Span::styled(
            format!(
                "  [D] Train Dragon (Cost: {} cookies) | [A] Cycle Aura",
                format_number(dragon_cost)
            ),
            view.fg(Color::Yellow),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "  Dragon fully trained! Press [A] to cycle aura.",
            view.fg(Color::Green),
        )));
    }

    lines.push(Line::from(Span::styled(
        format!(
            "Season: {} ({:.0}s left) - {} | [V] Cycle Season",
            state.season.name(),
            state.season_timer,
            state.season.description()
        ),
        view.fg(Color::LightCyan),
    )));
    lines.push(Line::from(""));

    // Grimoire
    lines.push(Line::from(Span::styled(
        format!(
            "Grimoire Magic: {:.1} / {:.1} ([Z/C/F] Cast Spell 1/2/3)",
            state.magic, state.max_magic
        ),
        view.bold_fg(Color::LightMagenta),
    )));
    for (idx, sp) in GRIMOIRE_SPELLS.iter().enumerate() {
        let key = match idx {
            0 => "Z",
            1 => "C",
            _ => "F",
        };
        lines.push(Line::from(Span::styled(
            format!(
                "  [{}] {} ({:.0} MP): {}",
                key, sp.name, sp.magic_cost, sp.description
            ),
            view.fg(Color::Gray),
        )));
    }
    lines.push(Line::from(""));

    // Garden 4x4
    let sel_seed = &GARDEN_SEEDS[state.garden_selected_seed];
    lines.push(Line::from(Span::styled(
        format!(
            "Garden (Seed: {} [M] Change | [P] Plant/Harvest Plot 1,1):",
            sel_seed.name
        ),
        view.bold_fg(Color::Green),
    )));
    let mut garden_row_str = String::from("  Plots: ");
    for r in 0..4 {
        for c in 0..4 {
            let cell_str = match state.garden_plots[r][c] {
                None => "[ . ]".to_string(),
                Some((sid, age)) => {
                    let sdef = &GARDEN_SEEDS[sid];
                    if age >= sdef.mature_ticks {
                        format!("[{}*]", &sdef.name[0..2])
                    } else {
                        format!("[{}:{}]", &sdef.name[0..1], age)
                    }
                }
            };
            garden_row_str.push_str(&cell_str);
            garden_row_str.push(' ');
        }
        if r == 1 {
            lines.push(Line::from(Span::styled(
                garden_row_str.clone(),
                view.fg(Color::LightGreen),
            )));
            garden_row_str = String::from("         ");
        }
    }
    lines.push(Line::from(Span::styled(
        garden_row_str,
        view.fg(Color::LightGreen),
    )));
    lines.push(Line::from(""));

    // Stock Market & Pantheon
    let mut stock_summary = String::from("Stocks ([K] Buy / [J] Sell Selected): ");
    for (idx, sym) in STOCK_SYMBOLS.iter().enumerate() {
        let mark = if idx == view.selected_stock_idx {
            "*"
        } else {
            ""
        };
        stock_summary.push_str(&format!(
            "{}{}:${:.0}({}) ",
            mark, sym, state.stock_prices[idx], state.stock_owned[idx]
        ));
    }
    lines.push(Line::from(Span::styled(
        stock_summary,
        view.fg(Color::Yellow),
    )));

    let spirit = &PANTHEON_SPIRITS[state.pantheon_spirit];
    lines.push(Line::from(Span::styled(
        format!(
            "Pantheon Spirit ([N] Cycle): {} - {}",
            spirit.name, spirit.description
        ),
        view.fg(Color::Cyan),
    )));

    let para = Paragraph::new(lines).wrap(Wrap { trim: false }).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Minigames, Dragon & Seasons "),
    );
    frame.render_widget(para, area);
}

fn render_tab_prestige(frame: &mut Frame, area: Rect, state: &GameState, view: &UiViewState) {
    let mut lines = vec![
        Line::from(Span::styled(
            format!(
                "Cookies Baked (This Run): {} ({})",
                format_number(state.cookies_baked_this_run),
                format_long(state.cookies_baked_this_run)
            ),
            view.fg(Color::White),
        )),
        Line::from(Span::styled(
            format!(
                "Cookies Baked (All Time): {} | Ascensions: {}",
                format_number(state.cookies_baked_all_time),
                state.ascensions
            ),
            view.fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            format!(
                "Prestige Level: {} (+{}% CpS) | Heavenly Chips: {} | Pending on Ascend: +{}",
                state.prestige_level,
                state.prestige_level,
                state.heavenly_chips,
                state.pending_prestige()
            ),
            view.bold_fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            format!(
                "Clicks: {} | Golden Clicks: {} | Wrinklers Popped: {} | Run Time: {}",
                state.total_clicks,
                state.golden_clicks,
                state.wrinklers_popped,
                format_duration(state.run_time_secs)
            ),
            view.fg(Color::Gray),
        )),
        Line::from(Span::styled(
            "Press [R] to Ascend | Press [H] to Buy Selected Heavenly Upgrade:",
            view.bold_fg(Color::LightMagenta),
        )),
    ];

    let sel = view
        .selected_heavenly_idx
        .min(HEAVENLY_UPGRADES.len().saturating_sub(1));
    for (idx, hupg) in HEAVENLY_UPGRADES.iter().enumerate() {
        let owned = state
            .heavenly_upgrades_purchased
            .get(idx)
            .copied()
            .unwrap_or(false);
        let tag = if owned {
            "[OWNED]"
        } else if state.heavenly_chips >= hupg.cost_chips {
            "[CAN BUY]"
        } else {
            "[LOCKED]"
        };
        let row = format!(
            " {} {} ({} HC) - {}",
            tag, hupg.name, hupg.cost_chips, hupg.description
        );
        let style = if idx == sel {
            view.highlight_style()
        } else if owned {
            view.fg(Color::Green)
        } else {
            view.fg(Color::Gray)
        };
        lines.push(Line::from(Span::styled(row, style)));
    }

    let para = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Statistics & Heavenly Prestige "),
    );
    frame.render_widget(para, area);
}

fn render_tab_achievements(frame: &mut Frame, area: Rect, state: &GameState, view: &UiViewState) {
    let unlocked_cnt = state.unlocked_achievements_count();
    let total_cnt = ACHIEVEMENTS.len();
    let pct = (unlocked_cnt as f64 / total_cnt as f64) * 100.0;

    let mut items = Vec::with_capacity(total_cnt + 1);
    items.push(ListItem::new(Line::from(Span::styled(
        format!(
            "Unlocked: {}/{} ({:.1}%) | Milk: {} ({:.0}%)",
            unlocked_cnt,
            total_cnt,
            pct,
            state.milk_name(),
            state.milk_progress() * 100.0
        ),
        view.bold_fg(Color::Cyan),
    ))));

    for (id, ach) in ACHIEVEMENTS.iter().enumerate() {
        let is_unlocked = state
            .achievements_unlocked
            .get(id)
            .copied()
            .unwrap_or(false);
        let prefix = if is_unlocked { "[✓]" } else { "[ ]" };
        let desc = if is_unlocked { ach.description } else { "???" };
        let line = format!("{} [{}] {} - {}", prefix, ach.category, ach.name, desc);
        let style = if is_unlocked {
            view.fg(Color::Green)
        } else {
            view.fg(Color::DarkGray)
        };
        items.push(ListItem::new(Line::from(Span::styled(line, style))));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Achievements & Milk "),
    );
    frame.render_widget(list, area);
}

fn render_tab_help(frame: &mut Frame, area: Rect, view: &UiViewState) {
    let lines = vec![
        Line::from(Span::styled(
            "Cookie-CLI Keyboard Controls",
            view.bold_fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from("  [SPACE] / [ENTER] : Click the Big Cookie"),
        Line::from("  [Up] / [Down]     : Select Building / Upgrade / Heavenly Item"),
        Line::from("  [Right] / [y]     : Buy or Sell selected Building"),
        Line::from("  [B]               : Toggle Store Mode (BUY / SELL)"),
        Line::from("  [X]               : Cycle Bulk Amount (x1 / x10 / x100 / Max)"),
        Line::from("  [U]               : Buy selected or cheapest available Upgrade"),
        Line::from("  [G]               : Click active Golden / Wrath Cookie shimmer"),
        Line::from("  [W]               : Pop all Wrinklers (+110% leeched cookies)"),
        Line::from("  [L]               : Spend Sugar Lump to level up selected Building"),
        Line::from("  [D] / [A]         : Train Krumblor Dragon / Cycle Dragon Aura"),
        Line::from("  [V]               : Cycle festive Season"),
        Line::from("  [Z] / [C] / [F]   : Cast Grimoire Spell 1 / 2 / 3"),
        Line::from("  [P] / [M]         : Plant or Harvest Garden Plot / Cycle Seed"),
        Line::from("  [K] / [J] / [N]   : Buy Stock / Sell Stock / Cycle Pantheon Spirit"),
        Line::from("  [R] / [H]         : Ascend (Prestige) / Buy Heavenly Upgrade"),
        Line::from("  [S] / [E] / [I]   : Save Game / Export Base64 / Import Base64"),
        Line::from("  [Tab] / [1-6]     : Switch Center Panel Tab"),
        Line::from("  [Q] / [Esc]       : Save and Quit"),
    ];

    let para = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Help & Shortcuts "),
    );
    frame.render_widget(para, area);
}

fn render_right_store(frame: &mut Frame, area: Rect, state: &GameState, view: &UiViewState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(10),
        ])
        .split(area);

    // 1. Store Mode & Bulk Amount selector
    let mode_color = match state.store_mode {
        StoreMode::Buy => Color::Green,
        StoreMode::Sell => Color::Red,
    };
    let controls_line = Line::from(vec![
        Span::styled(
            format!(" [B] Mode: {} ", state.store_mode.label()),
            view.bold_fg(mode_color),
        ),
        Span::raw(" | "),
        Span::styled(
            format!(" [X] Bulk: {} ", state.buy_amount.label()),
            view.bold_fg(Color::Yellow),
        ),
    ]);
    let controls = Paragraph::new(controls_line)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Store Controls "),
        );
    frame.render_widget(controls, chunks[0]);

    // 2. Quick Upgrades Strip
    let avail = state.available_upgrades();
    let mut upg_lines = Vec::new();
    if avail.is_empty() {
        upg_lines.push(Line::from(Span::styled(
            "No upgrades waiting.",
            view.fg(Color::DarkGray),
        )));
    } else {
        for &uid in avail.iter().take(3) {
            let u = &UPGRADES[uid];
            let can_afford = state.cookies >= u.cost;
            let st = if can_afford {
                view.bold_fg(Color::Green)
            } else {
                view.fg(Color::Gray)
            };
            upg_lines.push(Line::from(Span::styled(
                format!("[U] {} ({})", u.name, format_number(u.cost)),
                st,
            )));
        }
    }
    let upg_block = Paragraph::new(upg_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Next Upgrades ([U] Buy) "),
    );
    frame.render_widget(upg_block, chunks[1]);

    // 3. Building Store List
    let mut items = Vec::with_capacity(NUM_BUILDINGS);
    for (id, def) in BUILDINGS.iter().enumerate() {
        let owned = state.buildings[id];
        let (qty, price) = state.store_quote(id, state.store_mode, state.buy_amount);
        let can_transact = match state.store_mode {
            StoreMode::Buy => qty > 0 && state.cookies >= price,
            StoreMode::Sell => qty > 0,
        };
        let icon = if view.ascii_only {
            def.ascii_icon
        } else {
            def.icon
        };
        let row_str = format!(
            "{} {:<15} {:>3}x {:>9} [{}]",
            icon,
            def.name,
            qty,
            format_number(price),
            owned
        );
        let style = if id == view.selected_building {
            view.highlight_style()
        } else if can_transact {
            view.bold_fg(Color::Green)
        } else {
            view.fg(Color::DarkGray)
        };
        items.push(ListItem::new(Line::from(Span::styled(row_str, style))));
    }

    let store_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Buildings ([Up/Down] + [Right/B]) "),
    );
    frame.render_widget(store_list, chunks[2]);
}

fn render_modal(frame: &mut Frame, area: Rect, state: &GameState, view: &UiViewState) {
    let popup_area = centered_rect(70, 50, area);
    frame.render_widget(Clear, popup_area);

    let (title, lines) = match &view.modal {
        ModalState::None => return,
        ModalState::AscendConfirm => (
            " Confirm Ascension ",
            vec![
                Line::from(Span::styled(
                    "Are you ready to Ascend to the Heavenly Plane?",
                    view.bold_fg(Color::Yellow),
                )),
                Line::from(""),
                Line::from(format!(
                    "You will gain +{} Prestige Level and +{} Heavenly Chips.",
                    state.pending_prestige(),
                    state.pending_prestige()
                )),
                Line::from("Your current cookies and buildings will reset, but your prestige,"),
                Line::from("heavenly upgrades, achievements, and milk will carry over!"),
                Line::from(""),
                Line::from(Span::styled(
                    "Press [Y] or [Enter] to Ascend, or [N] / [Esc] to Cancel.",
                    view.bold_fg(Color::Green),
                )),
            ],
        ),
        ModalState::ResetConfirm => (
            " Confirm Hard Wipe ",
            vec![
                Line::from(Span::styled(
                    "WARNING: This will wipe your entire save!",
                    view.bold_fg(Color::Red),
                )),
                Line::from(""),
                Line::from("Press [Y] to confirm wipe, or [N] / [Esc] to cancel."),
            ],
        ),
        ModalState::ExportSave(code) => (
            " Export Save String (Base64) ",
            vec![
                Line::from(Span::styled(
                    "Copy the Base64 save string below:",
                    view.bold_fg(Color::Cyan),
                )),
                Line::from(""),
                Line::from(Span::styled(code.clone(), view.fg(Color::Yellow))),
                Line::from(""),
                Line::from("Press [Enter] or [Esc] to close."),
            ],
        ),
        ModalState::ImportSave(buf) => (
            " Import Save String (Base64) ",
            vec![
                Line::from(Span::styled(
                    "Paste or type your Base64 save string, then press [Enter]:",
                    view.bold_fg(Color::Cyan),
                )),
                Line::from(""),
                Line::from(Span::styled(format!("> {}_", buf), view.fg(Color::Yellow))),
                Line::from(""),
                Line::from("Press [Enter] to Import, [Backspace] to edit, or [Esc] to Cancel."),
            ],
        ),
    };

    let block = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(view.bold_fg(Color::Yellow)),
        );
    frame.render_widget(block, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_headless_render_all_tabs_and_modals() {
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).expect("terminal init");
        let mut state = GameState::with_seed(42);
        state.bake_cookies(1_000_000.0);
        state.buildings[0] = 10;
        state.buildings[1] = 5;
        state.spawn_shimmer();

        let mut anim = AnimState::new();
        anim.on_cookie_click(5.0, false);
        anim.push_toast("Wake and bake".to_string(), "Bake 1 cookie.".to_string());
        anim.update(0.1, state.cookies);

        let mut view = UiViewState::default();

        for tab in CenterTab::ALL {
            view.tab = tab;
            terminal
                .draw(|f| render(f, &state, &anim, &view))
                .expect("draw tab should succeed");
        }

        // Test modals
        for modal in [
            ModalState::AscendConfirm,
            ModalState::ResetConfirm,
            ModalState::ExportSave("CookieCLI|1|dGVzdA==|END".to_string()),
            ModalState::ImportSave("CookieCLI|1|".to_string()),
        ] {
            view.modal = modal;
            terminal
                .draw(|f| render(f, &state, &anim, &view))
                .expect("draw modal should succeed");
        }
    }

    #[test]
    fn test_headless_render_small_resolutions_and_ascii_mode() {
        let state = GameState::with_seed(1);
        let anim = AnimState::new();
        let view = UiViewState {
            ascii_only: true,
            no_color: true,
            ..Default::default()
        };

        for (w, h) in [(20, 6), (59, 17), (60, 18), (80, 24)] {
            let backend = TestBackend::new(w, h);
            let mut terminal = Terminal::new(backend).expect("init");
            terminal
                .draw(|f| render(f, &state, &anim, &view))
                .expect("draw small resolution should never panic");
        }
    }
}
