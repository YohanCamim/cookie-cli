//! Comprehensive acceptance tests verifying the full implementation of Cookie-CLI.
//! These tests prove all acceptance criteria for "refaire entierement Cookie clicker en CLI".

#[path = "../src/data/mod.rs"]
pub mod data;

#[path = "../src/format.rs"]
pub mod format;

#[path = "../src/engine/mod.rs"]
pub mod engine;

#[path = "../src/save.rs"]
pub mod save;

#[path = "../src/ui/mod.rs"]
pub mod ui;

#[path = "../src/app.rs"]
pub mod app;

use app::{App, AppConfig};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use data::{
    DragonAura, Season, BUILDINGS, GARDEN_SEEDS, GRIMOIRE_SPELLS, HEAVENLY_UPGRADES, NUM_BUILDINGS,
    PANTHEON_SPIRITS, STOCK_SYMBOLS, UPGRADES,
};
use engine::{Action, BuffKind, BuyAmount, GameState, StoreMode};
use ratatui::backend::TestBackend;
use ratatui::Terminal;
use save::{export_base64, import_base64, load_from_path, save_to_path};
use std::env;
use std::fs;
use std::time::{Duration, Instant};
use ui::anim::AnimState;
use ui::{render, CenterTab, ModalState, UiViewState};

fn make_key(code: KeyCode, kind: KeyEventKind) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind,
        state: KeyEventState::NONE,
    }
}

#[test]
fn test_cli_help_version_and_flags() {
    let bin = env!("CARGO_BIN_EXE_cookie-cli");

    // Test --help flag
    let help_output = std::process::Command::new(bin)
        .arg("--help")
        .output()
        .expect("failed to execute cookie-cli --help");
    assert!(help_output.status.success());
    let help_str = String::from_utf8_lossy(&help_output.stdout);
    assert!(help_str.contains("Full-featured terminal Cookie Clicker"));
    assert!(help_str.contains("--save-path"));
    assert!(help_str.contains("--fps"));
    assert!(help_str.contains("--no-color"));
    assert!(help_str.contains("--ascii"));
    assert!(help_str.contains("--reset"));
    assert!(help_str.contains("-h, --help"));
    assert!(help_str.contains("-V, --version"));

    // Test --version flag
    let ver_output = std::process::Command::new(bin)
        .arg("--version")
        .output()
        .expect("failed to execute cookie-cli --version");
    assert!(ver_output.status.success());
    let ver_str = String::from_utf8_lossy(&ver_output.stdout);
    assert!(ver_str.contains("cookie-cli 1.0.0"));

    // Test invalid flag exits with error
    let err_output = std::process::Command::new(bin)
        .arg("--invalid-flag-that-does-not-exist")
        .output()
        .expect("failed to execute cookie-cli");
    assert!(!err_output.status.success());
}

#[test]
fn test_canonical_buildings_and_cost_progression() {
    assert_eq!(BUILDINGS.len(), NUM_BUILDINGS);
    assert_eq!(NUM_BUILDINGS, 20);

    // Verify canonical building names and base costs
    assert_eq!(BUILDINGS[0].name, "Cursor");
    assert_eq!(BUILDINGS[0].base_cost, 15.0);
    assert_eq!(BUILDINGS[0].base_cps, 0.1);

    assert_eq!(BUILDINGS[1].name, "Grandma");
    assert_eq!(BUILDINGS[1].base_cost, 100.0);
    assert_eq!(BUILDINGS[1].base_cps, 1.0);

    assert_eq!(BUILDINGS[2].name, "Farm");
    assert_eq!(BUILDINGS[2].base_cost, 1_100.0);

    assert_eq!(BUILDINGS[19].name, "You");
    assert_eq!(
        BUILDINGS[19].base_cost,
        540_000_000_000_000_000_000_000_000.0
    );

    // Verify 1.15^n cost formula progression
    // n = 0: ceil(15 * 1.15^0) = 15
    // n = 1: ceil(15 * 1.15^1) = 18
    // n = 2: ceil(15 * 1.15^2) = 20
    assert_eq!(GameState::single_building_cost(0, 0), 15.0);
    assert_eq!(GameState::single_building_cost(0, 1), 18.0);
    assert_eq!(GameState::single_building_cost(0, 2), 20.0);
    assert_eq!(GameState::single_building_cost(1, 0), 100.0);
    assert_eq!(GameState::single_building_cost(1, 1), 115.0);
}

#[test]
fn test_building_bulk_buy_and_sell() {
    let mut state = GameState::with_seed(101);
    state.bake_cookies(10_000.0);

    // Bulk buy x1
    assert!(state.apply(Action::BuyBuilding(0, BuyAmount::One)));
    assert_eq!(state.buildings[0], 1);
    assert_eq!(state.cookies, 10_000.0 - 15.0);

    // Bulk buy x10
    let cost_10 = GameState::bulk_buy_cost(0, 1, 10);
    let cookies_before = state.cookies;
    assert!(state.apply(Action::BuyBuilding(0, BuyAmount::Ten)));
    assert_eq!(state.buildings[0], 11);
    assert!((state.cookies - (cookies_before - cost_10)).abs() < 1e-6);

    // Buy Max
    let mut small_state = GameState::with_seed(102);
    small_state.cookies = 50.0; // 15 + 18 = 33, next is 20 -> 53 > 50 -> 2 cursors
    assert!(small_state.apply(Action::BuyBuilding(0, BuyAmount::Max)));
    assert_eq!(small_state.buildings[0], 2);
    assert_eq!(small_state.cookies, 17.0);

    // Bulk sell
    let (sell_qty, refund) = state.store_quote(0, StoreMode::Sell, BuyAmount::One);
    assert_eq!(sell_qty, 1);
    assert!(refund > 0.0);
    let cookies_before_sell = state.cookies;
    assert!(state.apply(Action::SellBuilding(0, BuyAmount::One)));
    assert_eq!(state.buildings[0], 10);
    assert!((state.cookies - (cookies_before_sell + refund)).abs() < 1e-6);
}

#[test]
fn test_upgrades_synergies_and_kittens_calculation() {
    assert_eq!(UPGRADES[0].name, "Reinforced index finger");
    let mut state = GameState::with_seed(201);
    state.bake_cookies(1_000_000.0);

    // Buy 1 cursor and 1 grandma
    state.apply(Action::BuyBuilding(0, BuyAmount::One));
    state.apply(Action::BuyBuilding(1, BuyAmount::One));
    // Base: Cursor 0.1, Grandma 1.0 -> 1.1 CPS
    assert!((state.raw_cps() - 1.1).abs() < 1e-6);

    // Buy "Reinforced index finger" (upgrade 0, doubles cursor)
    assert!(state.apply(Action::BuyUpgrade(0)));
    // Cursor now 0.2, Grandma 1.0 -> 1.2 CPS
    assert!((state.raw_cps() - 1.2).abs() < 1e-6);
    // Click base was 1.0, doubled to 2.0
    assert!((state.cookies_per_click() - 2.0).abs() < 1e-6);

    // Test Thousand Fingers: buy 25 cursors, 10 grandmas, and Thousand Fingers (upgrade 3)
    state.buildings[0] = 25;
    state.apply(Action::BuyBuilding(1, BuyAmount::Ten)); // 11 grandmas
    assert!(state.apply(Action::BuyUpgrade(3)));
    let bonus = state.thousand_fingers_bonus();
    assert!(bonus > 0.0);
    assert!(state.cookies_per_click() > 2.0);

    // Test Kitten upgrades: Kitten helpers (upgrade 42)
    // Unlock 12 achievements to unlock Kitten helpers
    for i in 0..12 {
        state.achievements_unlocked[i] = true;
    }
    let milk = state.milk_progress();
    assert!(milk > 0.0);
    let cps_before_kitten = state.raw_cps();
    state.bake_cookies(10_000_000.0);
    assert!(state.apply(Action::BuyUpgrade(42)));
    assert!(state.raw_cps() > cps_before_kitten);
}

#[test]
fn test_achievements_unlocks_and_milk_flavors() {
    let mut state = GameState::with_seed(301);

    // Initial milk
    assert_eq!(state.unlocked_achievements_count(), 0);
    assert_eq!(state.milk_progress(), 0.0);
    assert_eq!(state.milk_name(), "Plain Milk");

    // Click cookie 10 times -> unlocks "Wake and bake" (Bake 1 cookie) and "Clicktastic" (10 clicks)
    for _ in 0..10 {
        state.apply(Action::ClickCookie);
    }
    assert!(state.achievements_unlocked[0]); // Wake and bake
    assert!(state.achievements_unlocked[15]); // Clicktastic (10 clicks)
    assert_eq!(state.unlocked_achievements_count(), 2);
    assert!((state.milk_progress() - 0.08).abs() < 1e-6);

    // Unlock many achievements to test milk tier progression
    for i in 0..10 {
        state.achievements_unlocked[i] = true;
    }
    // 10 * 0.04 = 0.40 -> Chocolate Milk
    assert_eq!(state.milk_name(), "Chocolate Milk");

    for i in 0..20 {
        state.achievements_unlocked[i] = true;
    }
    // 20 * 0.04 = 0.80 -> Raspberry Milk
    assert_eq!(state.milk_name(), "Raspberry Milk");

    for i in 0..30 {
        state.achievements_unlocked[i] = true;
    }
    // 30 * 0.04 = 1.20 -> Cosmic Caramel Milk
    assert_eq!(state.milk_name(), "Cosmic Caramel Milk");
}

#[test]
fn test_golden_wrath_cookies_shimmers_and_buffs() {
    let mut state = GameState::with_seed(401);
    state.bake_cookies(10_000.0);
    state.buildings[1] = 10; // 10 CPS

    // Spawn shimmer
    state.spawn_shimmer();
    assert!(state.active_shimmer.is_some());
    assert!(!state.active_shimmer.as_ref().unwrap().is_wrath);

    // Click golden cookie shimmer
    assert!(state.apply(Action::ClickShimmer));
    assert!(state.active_shimmer.is_none());
    assert_eq!(state.golden_clicks, 1);

    // Test buff decay
    let mut state_with_buff = GameState::with_seed(402);
    state_with_buff.buildings[1] = 10;
    // Apply Frenzy (x7)
    state_with_buff.apply(Action::CastSpell(1)); // summons shimmer
    state_with_buff.active_buffs.push(engine::ActiveBuff {
        kind: BuffKind::Frenzy,
        time_left: 10.0,
        max_time: 10.0,
    });
    assert!((state_with_buff.raw_cps() - 70.0).abs() < 1e-6);

    // Tick 5 seconds
    state_with_buff.tick(5.0);
    assert_eq!(state_with_buff.active_buffs.len(), 1);
    assert!((state_with_buff.active_buffs[0].time_left - 5.0).abs() < 1e-6);

    // Tick 6 more seconds -> buff expires
    state_with_buff.tick(6.0);
    assert_eq!(state_with_buff.active_buffs.len(), 0);
    assert!((state_with_buff.raw_cps() - 10.0).abs() < 1e-6);
}

#[test]
fn test_grandmapocalypse_wrinklers_and_elder_pledge() {
    let mut state = GameState::with_seed(501);
    state.buildings[1] = 50; // 50 Grandmas

    // Transition to Stage 1: Awoken (One Mind, upgrade 22)
    state.grandmapocalypse_stage = 1;
    assert_eq!(state.grandmapocalypse_status_name(), "Stage 1: Awoken");

    // Spawn wrinkler
    state.wrinklers[0].active = true;
    assert_eq!(state.active_wrinkler_count(), 1);
    assert_eq!(state.wrinkler_leech_fraction(), 0.05);

    // Wrinkler leeches 5% of CpS
    let raw = state.raw_cps();
    let effective = state.effective_cps();
    assert!((effective - raw * 0.95).abs() < 1e-6);

    // Tick 10 seconds to accumulate sucked cookies
    state.tick(10.0);
    assert!(state.wrinklers[0].cookies_sucked > 0.0);
    let sucked = state.wrinklers[0].cookies_sucked;

    // Pop wrinkler: returns 110% of sucked cookies
    let cookies_before = state.cookies;
    assert!(state.apply(Action::PopWrinkler(0)));
    assert!(!state.wrinklers[0].active);
    assert!((state.cookies - (cookies_before + sucked * 1.1)).abs() < 1e-6);
    assert_eq!(state.wrinklers_popped, 1);

    // Elder Pledge appeases grandmas for 300 seconds
    state.elder_pledge_timer = 300.0;
    assert_eq!(state.effective_grandmapocalypse_stage(), 0);
    assert!(state
        .grandmapocalypse_status_name()
        .contains("Elder Pledge"));

    // Tick 301 seconds -> Elder pledge expires
    state.tick(301.0);
    assert_eq!(state.effective_grandmapocalypse_stage(), 1);
}

#[test]
fn test_sugar_lumps_growth_and_building_leveling() {
    let mut state = GameState::with_seed(601);
    assert_eq!(state.sugar_lumps, 1);
    assert_eq!(state.building_levels[0], 0);

    // Level up Cursor (costs 1 lump for level 1)
    assert!(state.apply(Action::LevelUpBuilding(0)));
    assert_eq!(state.sugar_lumps, 0);
    assert_eq!(state.building_levels[0], 1);

    // Cannot level up without sugar lumps
    assert!(!state.apply(Action::LevelUpBuilding(0)));

    // Sugar lump ripening tick (1 lump every 180 seconds)
    state.tick(180.0);
    assert_eq!(state.sugar_lumps, 1);
}

#[test]
fn test_dragon_training_and_auras() {
    let mut state = GameState::with_seed(701);
    state.bake_cookies(100_000_000.0);
    assert_eq!(state.dragon_level, 0);
    assert_eq!(state.dragon_aura, DragonAura::None);

    // Train dragon to stage 1 -> Breath of Milk
    assert!(state.apply(Action::TrainDragon));
    assert_eq!(state.dragon_level, 1);
    assert_eq!(state.dragon_aura, DragonAura::BreathOfMilk);

    // Train dragon to stage 2 -> Dragon Cursor
    assert!(state.apply(Action::TrainDragon));
    assert_eq!(state.dragon_level, 2);
    assert_eq!(state.dragon_aura, DragonAura::DragonCursor);

    // Train dragon to stage 3 -> Elder Battalion
    assert!(state.apply(Action::TrainDragon));
    assert_eq!(state.dragon_level, 3);
    assert_eq!(state.dragon_aura, DragonAura::ElderBattalion);

    // Train dragon to stage 4 -> Radiant Appetite (2x CpS)
    assert!(state.apply(Action::TrainDragon));
    assert_eq!(state.dragon_level, 4);
    assert_eq!(state.dragon_aura, DragonAura::RadiantAppetite);

    // Cycle aura through unlocked auras
    assert!(state.apply(Action::CycleDragonAura));
    assert_ne!(state.dragon_aura, DragonAura::RadiantAppetite);
}

#[test]
fn test_seasons_switching_and_effects() {
    let mut state = GameState::with_seed(801);
    state.bake_cookies(1_000_000.0);
    assert_eq!(state.season, Season::None);

    // Set Season to Christmas (+5% CpS)
    assert!(state.apply(Action::SetSeason(Season::Christmas)));
    assert_eq!(state.season, Season::Christmas);
    assert_eq!(state.season.cps_multiplier(), 1.05);

    // Set Season to Valentine (+5% CpS)
    assert!(state.apply(Action::SetSeason(Season::Valentines)));
    assert_eq!(state.season, Season::Valentines);
    assert_eq!(state.season.cps_multiplier(), 1.05);

    // Cycle season
    assert!(state.apply(Action::CycleSeason));
    assert_ne!(state.season, Season::Valentines);
}

#[test]
fn test_minigames_comprehensive() {
    let mut state = GameState::with_seed(901);
    state.bake_cookies(50_000_000.0);
    state.buildings[7] = 10; // 10 Wizard Towers -> grants mana

    // 1. Garden
    assert_eq!(GARDEN_SEEDS.len(), 4);
    assert!(state.apply(Action::GardenSelectSeed(1))); // Bakeberry
    assert!(state.apply(Action::GardenInteractPlot(0, 0))); // Plant
    assert!(state.garden_plots[0][0].is_some());
    // Age seed
    state.tick(100.0);
    assert!(state.apply(Action::GardenInteractPlot(0, 0))); // Harvest mature plant
    assert!(state.garden_plots[0][0].is_none());

    // 2. Grimoire spells
    assert_eq!(GRIMOIRE_SPELLS.len(), 3);
    state.magic = 30.0;
    // Cast Spell 0: Conjure Baked Goods
    assert!(state.apply(Action::CastSpell(0)));
    // Cast Spell 1: Force the Hand of Fate (summons shimmer)
    state.magic = 30.0;
    assert!(state.apply(Action::CastSpell(1)));
    assert!(state.active_shimmer.is_some());
    // Cast Spell 2: Stretch Time
    state.magic = 30.0;
    assert!(state.apply(Action::CastSpell(2)));

    // 3. Stock Market
    assert_eq!(STOCK_SYMBOLS.len(), 6);
    let initial_shares = state.stock_owned[0];
    assert!(state.apply(Action::BuyStock(0)));
    assert_eq!(state.stock_owned[0], initial_shares + 1);
    assert!(state.apply(Action::SellStock(0)));
    assert_eq!(state.stock_owned[0], initial_shares);

    // 4. Pantheon spirits
    assert_eq!(PANTHEON_SPIRITS.len(), 4);
    assert!(state.apply(Action::CyclePantheonSpirit));
    assert_ne!(state.pantheon_spirit, 0);
}

#[test]
fn test_ascension_prestige_and_heavenly_upgrades() {
    let mut state = GameState::with_seed(1001);
    // Bake 27 trillion cookies -> cbrt(27) = 3 prestige levels
    state.bake_cookies(27_000_000_000_000.0);
    assert_eq!(state.pending_prestige(), 3);

    // Ascend
    assert!(state.apply(Action::Ascend));
    assert_eq!(state.prestige_level, 3);
    assert_eq!(state.heavenly_chips, 3);
    assert_eq!(state.ascensions, 1);
    assert_eq!(state.cookies, 0.0);

    // Buy Heavenly Upgrade 0: Legacy (cost 1 chip)
    assert_eq!(HEAVENLY_UPGRADES[0].name, "Legacy");
    assert!(state.apply(Action::BuyHeavenlyUpgrade(0)));
    assert_eq!(state.heavenly_chips, 2);
    assert!(state.heavenly_upgrades_purchased[0]);

    // Buy Heavenly Upgrade 3: Starter kit (cost 50 chips)
    state.heavenly_chips += 50;
    assert_eq!(HEAVENLY_UPGRADES[3].name, "Starter kit");
    assert!(state.apply(Action::BuyHeavenlyUpgrade(3)));

    // Subsequent ascension with Starter Kit gives 10 cursors automatically
    state.bake_cookies(64_000_000_000_000.0);
    assert!(state.apply(Action::Ascend));
    assert_eq!(state.buildings[0], 10);
}

#[test]
fn test_save_load_offline_progress_and_base64_import_export() {
    let temp_dir = env::temp_dir().join("cookie_cli_acceptance_save");
    let _ = fs::remove_dir_all(&temp_dir);
    let save_file = temp_dir.join("save.json");

    let mut state = GameState::with_seed(1101);
    state.bake_cookies(500_000.0);
    state.buildings[0] = 25;
    state.buildings[1] = 10;
    state.prestige_level = 5;

    // Save to path (creates JSON save)
    save_to_path(&state, &save_file).expect("save must succeed");
    assert!(save_file.exists());

    // Save again to verify .bak backup creation
    state.bake_cookies(50_000.0);
    save_to_path(&state, &save_file).expect("save must succeed");
    let bak_file = save_file.with_extension("json.bak");
    assert!(bak_file.exists());

    // Load from path
    let (loaded, offline_gain) = load_from_path(&save_file).expect("load must succeed");
    assert!((loaded.cookies - 550_000.0).abs() < 1e-6);
    assert_eq!(loaded.buildings[0], 25);
    assert_eq!(loaded.buildings[1], 10);
    assert_eq!(loaded.prestige_level, 5);
    assert!(offline_gain >= 0.0);

    // Base64 export & import roundtrip
    let export_str = export_base64(&state).expect("export base64 must succeed");
    assert!(export_str.starts_with("CookieCLI|1|"));
    assert!(export_str.ends_with("|END"));

    let imported = import_base64(&export_str).expect("import base64 must succeed");
    assert!((imported.cookies - state.cookies).abs() < 1e-6);
    assert_eq!(imported.buildings[0], 25);
    assert_eq!(imported.buildings[1], 10);

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_anti_autoclick_and_input_handling() {
    let state = GameState::with_seed(1201);
    let config = AppConfig {
        save_path: std::path::PathBuf::from("test_app_unused.json"),
        fps: 60,
        ascii_only: false,
        no_color: false,
    };
    let mut app = App::new(state, config);
    let t0 = Instant::now();

    // Release and Repeat events must NOT increment clicks
    app.handle_key_event(make_key(KeyCode::Char(' '), KeyEventKind::Release), t0);
    app.handle_key_event(make_key(KeyCode::Char(' '), KeyEventKind::Repeat), t0);
    assert_eq!(app.state.total_clicks, 0);

    // Initial click succeeds
    app.handle_key_event(make_key(KeyCode::Char(' '), KeyEventKind::Press), t0);
    assert_eq!(app.state.total_clicks, 1);

    // Click after 20ms (< 50ms anti-autoclick) is filtered
    app.handle_key_event(
        make_key(KeyCode::Char(' '), KeyEventKind::Press),
        t0 + Duration::from_millis(20),
    );
    assert_eq!(app.state.total_clicks, 1);

    // Click after 55ms (> 50ms cooldown) succeeds
    app.handle_key_event(
        make_key(KeyCode::Char(' '), KeyEventKind::Press),
        t0 + Duration::from_millis(55),
    );
    assert_eq!(app.state.total_clicks, 2);

    // Toggle Store mode key 'B'
    app.handle_key_event(
        make_key(KeyCode::Char('b'), KeyEventKind::Press),
        t0 + Duration::from_millis(100),
    );
    assert_eq!(app.state.store_mode, StoreMode::Sell);

    // Cycle Bulk amount key 'X'
    app.handle_key_event(
        make_key(KeyCode::Char('x'), KeyEventKind::Press),
        t0 + Duration::from_millis(150),
    );
    assert_eq!(app.state.buy_amount, BuyAmount::Ten);

    // Tab key switches center panel tabs
    app.handle_key_event(
        make_key(KeyCode::Tab, KeyEventKind::Press),
        t0 + Duration::from_millis(200),
    );
    assert_eq!(app.view.tab, CenterTab::Upgrades);
}

#[test]
fn test_tui_rendering_all_tabs_modals_ascii_and_sizes() {
    let mut state = GameState::with_seed(1301);
    state.bake_cookies(10_000_000.0);
    state.buildings[0] = 10;
    state.buildings[1] = 5;
    state.buildings[7] = 3;
    state.spawn_shimmer();

    let mut anim = AnimState::new();
    anim.on_cookie_click(10.0, false);
    anim.push_toast("Cookie Master".to_string(), "Bake 1M cookies.".to_string());
    anim.update(0.1, state.cookies);

    // 1. Render all tabs in standard 120x40 terminal
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).expect("init terminal");
    let mut view = UiViewState::default();

    for tab in CenterTab::ALL {
        view.tab = tab;
        terminal
            .draw(|f| render(f, &state, &anim, &view))
            .expect("drawing tab must not panic");
    }

    // 2. Render all modal dialogs
    for modal in [
        ModalState::AscendConfirm,
        ModalState::ResetConfirm,
        ModalState::ExportSave("CookieCLI|1|VGZXdA==|END".to_string()),
        ModalState::ImportSave("CookieCLI|1|".to_string()),
    ] {
        view.modal = modal;
        terminal
            .draw(|f| render(f, &state, &anim, &view))
            .expect("drawing modal must not panic");
    }

    // 3. Render in ASCII-only and no-color mode
    view.modal = ModalState::None;
    view.ascii_only = true;
    view.no_color = true;
    terminal
        .draw(|f| render(f, &state, &anim, &view))
        .expect("drawing in ascii-only no-color mode must not panic");

    // 4. Render on small resolutions down to 20x6 without crash
    for (w, h) in [(20, 6), (40, 15), (60, 20), (80, 24)] {
        let small_backend = TestBackend::new(w, h);
        let mut small_term = Terminal::new(small_backend).expect("small term init");
        small_term
            .draw(|f| render(f, &state, &anim, &view))
            .expect("drawing small resolution must not panic");
    }
}
