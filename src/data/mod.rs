//! Static canonical definitions for Cookie-CLI: buildings, upgrades, achievements,
//! prestige upgrades, dragon stages/auras, seasons, minigame items, and news ticker lines.

pub const NUM_BUILDINGS: usize = 20;

#[derive(Debug, Clone, Copy)]
pub struct BuildingDef {
    pub id: usize,
    pub name: &'static str,
    pub short_name: &'static str,
    pub icon: &'static str,
    pub ascii_icon: &'static str,
    pub base_cost: f64,
    pub base_cps: f64,
    pub description: &'static str,
}

pub const BUILDINGS: [BuildingDef; NUM_BUILDINGS] = [
    BuildingDef {
        id: 0,
        name: "Cursor",
        short_name: "Crsr",
        icon: "👆",
        ascii_icon: "[>]",
        base_cost: 15.0,
        base_cps: 0.1,
        description: "Autoclicks once every 10 seconds.",
    },
    BuildingDef {
        id: 1,
        name: "Grandma",
        short_name: "Grma",
        icon: "👵",
        ascii_icon: "[G]",
        base_cost: 100.0,
        base_cps: 1.0,
        description: "A nice grandma to bake more cookies.",
    },
    BuildingDef {
        id: 2,
        name: "Farm",
        short_name: "Farm",
        icon: "🌾",
        ascii_icon: "[F]",
        base_cost: 1_100.0,
        base_cps: 8.0,
        description: "Grows cookie plants from cookie seeds.",
    },
    BuildingDef {
        id: 3,
        name: "Mine",
        short_name: "Mine",
        icon: "⛏️",
        ascii_icon: "[M]",
        base_cost: 12_000.0,
        base_cps: 47.0,
        description: "Mines out cookie dough and chocolate chips.",
    },
    BuildingDef {
        id: 4,
        name: "Factory",
        short_name: "Fcty",
        icon: "🏭",
        ascii_icon: "[#]",
        base_cost: 130_000.0,
        base_cps: 260.0,
        description: "Produces large quantities of cookies.",
    },
    BuildingDef {
        id: 5,
        name: "Bank",
        short_name: "Bank",
        icon: "🏦",
        ascii_icon: "[$]",
        base_cost: 1_400_000.0,
        base_cps: 1_400.0,
        description: "Generates cookies from interest.",
    },
    BuildingDef {
        id: 6,
        name: "Temple",
        short_name: "Tmpl",
        icon: "🏛️",
        ascii_icon: "[^]",
        base_cost: 20_000_000.0,
        base_cps: 7_800.0,
        description: "Full of precious, ancient chocolate.",
    },
    BuildingDef {
        id: 7,
        name: "Wizard Tower",
        short_name: "Wzrd",
        icon: "🧙",
        ascii_icon: "[*]",
        base_cost: 330_000_000.0,
        base_cps: 44_000.0,
        description: "Summons cookies with magical spells.",
    },
    BuildingDef {
        id: 8,
        name: "Shipment",
        short_name: "Ship",
        icon: "🚀",
        ascii_icon: "[S]",
        base_cost: 5_100_000_000.0,
        base_cps: 260_000.0,
        description: "Brings in fresh cookies from the cookie planet.",
    },
    BuildingDef {
        id: 9,
        name: "Alchemy Lab",
        short_name: "Alch",
        icon: "⚗️",
        ascii_icon: "[A]",
        base_cost: 75_000_000_000.0,
        base_cps: 1_600_000.0,
        description: "Turns gold into cookies!",
    },
    BuildingDef {
        id: 10,
        name: "Portal",
        short_name: "Prtl",
        icon: "🌀",
        ascii_icon: "[O]",
        base_cost: 1_000_000_000_000.0,
        base_cps: 10_000_000.0,
        description: "Opens a door to the Cookieverse.",
    },
    BuildingDef {
        id: 11,
        name: "Time Machine",
        short_name: "Time",
        icon: "⏳",
        ascii_icon: "[T]",
        base_cost: 14_000_000_000_000.0,
        base_cps: 65_000_000.0,
        description: "Brings cookies from the past, before they were even eaten.",
    },
    BuildingDef {
        id: 12,
        name: "Antimatter Condenser",
        short_name: "Anti",
        icon: "⚛️",
        ascii_icon: "[%]",
        base_cost: 170_000_000_000_000.0,
        base_cps: 430_000_000.0,
        description: "Condenses the antimatter in the universe into cookies.",
    },
    BuildingDef {
        id: 13,
        name: "Prism",
        short_name: "Prsm",
        icon: "🌈",
        ascii_icon: "[/]",
        base_cost: 2_100_000_000_000_000.0,
        base_cps: 2_900_000_000.0,
        description: "Converts light itself into cookies.",
    },
    BuildingDef {
        id: 14,
        name: "Chancemaker",
        short_name: "Chnc",
        icon: "🍀",
        ascii_icon: "[?]",
        base_cost: 26_000_000_000_000_000.0,
        base_cps: 21_000_000_000.0,
        description: "Generates cookies out of thin air through sheer luck.",
    },
    BuildingDef {
        id: 15,
        name: "Fractal Engine",
        short_name: "FrcE",
        icon: "❄️",
        ascii_icon: "[&]",
        base_cost: 310_000_000_000_000_000.0,
        base_cps: 150_000_000_000.0,
        description: "Turns cookies into even more cookies.",
    },
    BuildingDef {
        id: 16,
        name: "Javascript Console",
        short_name: "JS",
        icon: "💻",
        ascii_icon: "[J]",
        base_cost: 71_000_000_000_000_000_000.0,
        base_cps: 1_100_000_000_000.0,
        description: "Creates cookies from the very code this game wasn't written in.",
    },
    BuildingDef {
        id: 17,
        name: "Idleverse",
        short_name: "Idle",
        icon: "🌌",
        ascii_icon: "[I]",
        base_cost: 12_000_000_000_000_000_000_000.0,
        base_cps: 8_300_000_000_000.0,
        description: "Hijacks parallel universes running other idle games.",
    },
    BuildingDef {
        id: 18,
        name: "Cortex Baker",
        short_name: "Crtx",
        icon: "🧠",
        ascii_icon: "[B]",
        base_cost: 1_900_000_000_000_000_000_000_000.0,
        base_cps: 64_000_000_000_000.0,
        description: "Planet-sized brains that dream cookies into existence.",
    },
    BuildingDef {
        id: 19,
        name: "You",
        short_name: "You",
        icon: "🪞",
        ascii_icon: "[Y]",
        base_cost: 540_000_000_000_000_000_000_000_000.0,
        base_cps: 510_000_000_000_000.0,
        description: "Clones of yourself to help bake cookies around the clock.",
    },
];

#[derive(Debug, Clone, Copy)]
pub enum UpgradeUnlock {
    BuildingCount { building_id: usize, count: u32 },
    TotalClicks(u64),
    CookiesBaked(f64),
    AchievementCount(usize),
    GrandmapocalypseStage(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum UpgradeEffect {
    /// Doubles (or multiplies) a specific building's CPS (and click base if Cursor).
    BuildingMultiplier { building_id: usize, factor: f64 },
    /// Adds flat cookies per non-cursor building owned to both Cursor CPS and clicking.
    ThousandFingers { bonus_per_non_cursor: f64 },
    /// Clicking gains a percentage of total CPS.
    ClickCpsPercent { pct: f64 },
    /// Kitten milk-based CPS multiplier (`1.0 + milk * milk_factor`).
    Kitten { milk_factor: f64 },
    /// Grandma synergy: Grandmas are 2x as efficient and target building gets +1% per Grandma.
    GrandmaSynergy { target_building: usize },
    /// Synergy between two buildings: A gets +5% per B, B gets +0.1% per A.
    Synergy {
        building_a: usize,
        building_b: usize,
    },
    /// Global CPS multiplier (e.g., flavored cookies).
    GlobalCpsMultiplier { factor: f64 },
    /// Golden Cookie frequency & duration multiplier.
    GoldenCookieBoost {
        freq_factor: f64,
        effect_duration_factor: f64,
    },
    /// Advances or alters Grandmapocalypse state.
    GrandmapocalypseTransition { target_stage: u8, pledge: bool },
}

#[derive(Debug, Clone, Copy)]
pub struct UpgradeDef {
    pub id: usize,
    pub name: &'static str,
    pub cost: f64,
    pub description: &'static str,
    pub unlock: UpgradeUnlock,
    pub effect: UpgradeEffect,
}

pub const UPGRADES: &[UpgradeDef] = &[
    // --- Cursor Tiered & Finger Upgrades ---
    UpgradeDef {
        id: 0,
        name: "Reinforced index finger",
        cost: 100.0,
        description: "The mouse and cursors are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 0,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 0,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 1,
        name: "Carpal tunnel prevention cream",
        cost: 500.0,
        description: "The mouse and cursors are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 0,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 0,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 2,
        name: "Ambidextrous",
        cost: 10_000.0,
        description: "The mouse and cursors are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 0,
            count: 10,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 0,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 3,
        name: "Thousand fingers",
        cost: 100_000.0,
        description: "The mouse and cursors gain +0.1 cookies for each non-cursor object owned.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 0,
            count: 25,
        },
        effect: UpgradeEffect::ThousandFingers {
            bonus_per_non_cursor: 0.1,
        },
    },
    UpgradeDef {
        id: 4,
        name: "Million fingers",
        cost: 10_000_000.0,
        description: "The mouse and cursors gain +0.5 cookies for each non-cursor object owned.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 0,
            count: 50,
        },
        effect: UpgradeEffect::ThousandFingers {
            bonus_per_non_cursor: 0.5,
        },
    },
    UpgradeDef {
        id: 5,
        name: "Billion fingers",
        cost: 100_000_000.0,
        description: "The mouse and cursors gain +5 cookies for each non-cursor object owned.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 0,
            count: 100,
        },
        effect: UpgradeEffect::ThousandFingers {
            bonus_per_non_cursor: 5.0,
        },
    },
    // --- Mouse Clicking Upgrades ---
    UpgradeDef {
        id: 6,
        name: "Plastic mouse",
        cost: 50_000.0,
        description: "Clicking gains +1% of your CpS.",
        unlock: UpgradeUnlock::TotalClicks(100),
        effect: UpgradeEffect::ClickCpsPercent { pct: 0.01 },
    },
    UpgradeDef {
        id: 7,
        name: "Iron mouse",
        cost: 5_000_000.0,
        description: "Clicking gains +1% of your CpS.",
        unlock: UpgradeUnlock::TotalClicks(500),
        effect: UpgradeEffect::ClickCpsPercent { pct: 0.01 },
    },
    UpgradeDef {
        id: 8,
        name: "Titanium mouse",
        cost: 500_000_000.0,
        description: "Clicking gains +1% of your CpS.",
        unlock: UpgradeUnlock::TotalClicks(1_000),
        effect: UpgradeEffect::ClickCpsPercent { pct: 0.01 },
    },
    UpgradeDef {
        id: 9,
        name: "Adamantium mouse",
        cost: 50_000_000_000.0,
        description: "Clicking gains +1% of your CpS.",
        unlock: UpgradeUnlock::TotalClicks(2_500),
        effect: UpgradeEffect::ClickCpsPercent { pct: 0.01 },
    },
    // --- Grandma Tiered Upgrades ---
    UpgradeDef {
        id: 10,
        name: "Forwards from grandma",
        cost: 1_000.0,
        description: "Grandmas are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 1,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 1,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 11,
        name: "Steel-plated rolling pins",
        cost: 5_000.0,
        description: "Grandmas are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 1,
            count: 5,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 1,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 12,
        name: "Lubricated dentures",
        cost: 50_000.0,
        description: "Grandmas are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 1,
            count: 25,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 1,
            factor: 2.0,
        },
    },
    // --- Farm Tiered Upgrades ---
    UpgradeDef {
        id: 13,
        name: "Cheap hoes",
        cost: 11_000.0,
        description: "Farms are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 2,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 2,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 14,
        name: "Fertilizer",
        cost: 55_000.0,
        description: "Farms are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 2,
            count: 5,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 2,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 15,
        name: "Cookie trees",
        cost: 550_000.0,
        description: "Farms are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 2,
            count: 25,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 2,
            factor: 2.0,
        },
    },
    // --- Mine Tiered Upgrades ---
    UpgradeDef {
        id: 16,
        name: "Sugar gas",
        cost: 120_000.0,
        description: "Mines are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 3,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 3,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 17,
        name: "Megadrill",
        cost: 600_000.0,
        description: "Mines are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 3,
            count: 5,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 3,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 18,
        name: "Ultradrill",
        cost: 6_000_000.0,
        description: "Mines are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 3,
            count: 25,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 3,
            factor: 2.0,
        },
    },
    // --- Factory Tiered Upgrades ---
    UpgradeDef {
        id: 19,
        name: "Sturdier conveyor belts",
        cost: 1_300_000.0,
        description: "Factories are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 4,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 4,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 20,
        name: "Child labor",
        cost: 6_500_000.0,
        description: "Factories are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 4,
            count: 5,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 4,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 21,
        name: "Sweatshop",
        cost: 65_000_000.0,
        description: "Factories are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 4,
            count: 25,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 4,
            factor: 2.0,
        },
    },
    // --- Bank, Temple, Wizard Tower, Shipment, Alchemy, Portal, Time Machine, Antimatter, Prism, Chancemaker, Fractal, JS, Idleverse, Cortex, You Tier 1 Upgrades ---
    UpgradeDef {
        id: 22,
        name: "Taller tellers",
        cost: 14_000_000.0,
        description: "Banks are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 5,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 5,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 23,
        name: "Golden idols",
        cost: 200_000_000.0,
        description: "Temples are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 6,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 6,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 24,
        name: "Pointier hats",
        cost: 3_300_000_000.0,
        description: "Wizard towers are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 7,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 7,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 25,
        name: "Vanilla nebulae",
        cost: 51_000_000_000.0,
        description: "Shipments are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 8,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 8,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 26,
        name: "Antimony",
        cost: 750_000_000_000.0,
        description: "Alchemy labs are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 9,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 9,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 27,
        name: "Ancient tablet",
        cost: 10_000_000_000_000.0,
        description: "Portals are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 10,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 10,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 28,
        name: "Flux capacitors",
        cost: 140_000_000_000_000.0,
        description: "Time machines are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 11,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 11,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 29,
        name: "Sugar bosons",
        cost: 1_700_000_000_000_000.0,
        description: "Antimatter condensers are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 12,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 12,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 30,
        name: "Gem polish",
        cost: 21_000_000_000_000_000.0,
        description: "Prisms are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 13,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 13,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 31,
        name: "Your lucky cookie",
        cost: 260_000_000_000_000_000.0,
        description: "Chancemakers are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 14,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 14,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 32,
        name: "Metabakeries",
        cost: 3_100_000_000_000_000_000.0,
        description: "Fractal engines are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 15,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 15,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 33,
        name: "The JavaScript console for dummies",
        cost: 710_000_000_000_000_000_000.0,
        description: "Javascript consoles are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 16,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 16,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 34,
        name: "Manifest destiny",
        cost: 120_000_000_000_000_000_000_000.0,
        description: "Idleverses are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 17,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 17,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 35,
        name: "Principled neuralilds",
        cost: 19_000_000_000_000_000_000_000_000.0,
        description: "Cortex bakers are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 18,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 18,
            factor: 2.0,
        },
    },
    UpgradeDef {
        id: 36,
        name: "Cloning vats",
        cost: 5_400_000_000_000_000_000_000_000_000.0,
        description: "You are twice as efficient.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 19,
            count: 1,
        },
        effect: UpgradeEffect::BuildingMultiplier {
            building_id: 19,
            factor: 2.0,
        },
    },
    // --- Grandma Synergies ---
    UpgradeDef {
        id: 37,
        name: "Farmer grandmas",
        cost: 55_000.0,
        description: "Grandmas are twice as efficient; Farms gain +1% CpS per grandma.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 2,
            count: 15,
        },
        effect: UpgradeEffect::GrandmaSynergy { target_building: 2 },
    },
    UpgradeDef {
        id: 38,
        name: "Miner grandmas",
        cost: 600_000.0,
        description: "Grandmas are twice as efficient; Mines gain +1% CpS per grandma.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 3,
            count: 15,
        },
        effect: UpgradeEffect::GrandmaSynergy { target_building: 3 },
    },
    UpgradeDef {
        id: 39,
        name: "Worker grandmas",
        cost: 6_500_000.0,
        description: "Grandmas are twice as efficient; Factories gain +1% CpS per grandma.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 4,
            count: 15,
        },
        effect: UpgradeEffect::GrandmaSynergy { target_building: 4 },
    },
    // --- Building Synergies ---
    UpgradeDef {
        id: 40,
        name: "Future almanacs",
        cost: 1_000_000_000_000.0,
        description: "Farms gain +5% CpS per Time Machine; Time Machines gain +0.1% CpS per Farm.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 11,
            count: 15,
        },
        effect: UpgradeEffect::Synergy {
            building_a: 2,
            building_b: 11,
        },
    },
    UpgradeDef {
        id: 41,
        name: "Seismic magic",
        cost: 50_000_000_000.0,
        description: "Mines gain +5% CpS per Wizard Tower; Wizard Towers gain +0.1% CpS per Mine.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 7,
            count: 15,
        },
        effect: UpgradeEffect::Synergy {
            building_a: 3,
            building_b: 7,
        },
    },
    // --- Kitten Upgrades (Milk-scaled) ---
    UpgradeDef {
        id: 42,
        name: "Kitten helpers",
        cost: 9_000_000.0,
        description: "You gain more CpS the more milk you have (factor 0.1).",
        unlock: UpgradeUnlock::AchievementCount(12),
        effect: UpgradeEffect::Kitten { milk_factor: 0.1 },
    },
    UpgradeDef {
        id: 43,
        name: "Kitten workers",
        cost: 9_000_000_000.0,
        description: "You gain more CpS the more milk you have (factor 0.125).",
        unlock: UpgradeUnlock::AchievementCount(25),
        effect: UpgradeEffect::Kitten { milk_factor: 0.125 },
    },
    UpgradeDef {
        id: 44,
        name: "Kitten engineers",
        cost: 90_000_000_000_000.0,
        description: "You gain more CpS the more milk you have (factor 0.15).",
        unlock: UpgradeUnlock::AchievementCount(35),
        effect: UpgradeEffect::Kitten { milk_factor: 0.15 },
    },
    // --- Flavored Cookie Global Multipliers ---
    UpgradeDef {
        id: 45,
        name: "Plain cookies",
        cost: 999_999.0,
        description: "Cookie production multiplier +5%.",
        unlock: UpgradeUnlock::CookiesBaked(50_000.0),
        effect: UpgradeEffect::GlobalCpsMultiplier { factor: 1.05 },
    },
    UpgradeDef {
        id: 46,
        name: "Oatmeal raisin cookies",
        cost: 9_999_999.0,
        description: "Cookie production multiplier +5%.",
        unlock: UpgradeUnlock::CookiesBaked(500_000.0),
        effect: UpgradeEffect::GlobalCpsMultiplier { factor: 1.05 },
    },
    UpgradeDef {
        id: 47,
        name: "Chocolate chip cookies",
        cost: 99_999_999.0,
        description: "Cookie production multiplier +10%.",
        unlock: UpgradeUnlock::CookiesBaked(5_000_000.0),
        effect: UpgradeEffect::GlobalCpsMultiplier { factor: 1.10 },
    },
    // --- Golden Cookie Upgrades ---
    UpgradeDef {
        id: 48,
        name: "Lucky day",
        cost: 777_777_777.0,
        description: "Golden cookies appear twice as often and last twice as long.",
        unlock: UpgradeUnlock::CookiesBaked(77_777_777.0),
        effect: UpgradeEffect::GoldenCookieBoost {
            freq_factor: 2.0,
            effect_duration_factor: 1.0,
        },
    },
    UpgradeDef {
        id: 49,
        name: "Get lucky",
        cost: 77_777_777_777_777.0,
        description: "Golden cookie effects last twice as long.",
        unlock: UpgradeUnlock::CookiesBaked(777_777_777_777.0),
        effect: UpgradeEffect::GoldenCookieBoost {
            freq_factor: 1.0,
            effect_duration_factor: 2.0,
        },
    },
    // --- Grandmapocalypse Research Upgrades ---
    UpgradeDef {
        id: 50,
        name: "One mind",
        cost: 16_000_000_000_000.0,
        description: "Awakens the Grandmapocalypse (Stage 1: Awoken). Wrinklers may appear.",
        unlock: UpgradeUnlock::BuildingCount {
            building_id: 1,
            count: 15,
        },
        effect: UpgradeEffect::GrandmapocalypseTransition {
            target_stage: 1,
            pledge: false,
        },
    },
    UpgradeDef {
        id: 51,
        name: "Communal brainsweep",
        cost: 160_000_000_000_000.0,
        description: "Advances the Grandmapocalypse (Stage 2: Displeased).",
        unlock: UpgradeUnlock::GrandmapocalypseStage(1),
        effect: UpgradeEffect::GrandmapocalypseTransition {
            target_stage: 2,
            pledge: false,
        },
    },
    UpgradeDef {
        id: 52,
        name: "Elder Pact",
        cost: 1_600_000_000_000_000.0,
        description: "Full Grandmapocalypse (Stage 3: Angered). Grandmas gain massive power.",
        unlock: UpgradeUnlock::GrandmapocalypseStage(2),
        effect: UpgradeEffect::GrandmapocalypseTransition {
            target_stage: 3,
            pledge: false,
        },
    },
    UpgradeDef {
        id: 53,
        name: "Elder Pledge",
        cost: 1_000_000_000.0,
        description:
            "Appeases the grandmatriarchs temporarily, stopping Wrath Cookies and Wrinklers.",
        unlock: UpgradeUnlock::GrandmapocalypseStage(3),
        effect: UpgradeEffect::GrandmapocalypseTransition {
            target_stage: 3,
            pledge: true,
        },
    },
];

#[derive(Debug, Clone, Copy)]
pub struct HeavenlyUpgradeDef {
    pub id: usize,
    pub name: &'static str,
    pub cost_chips: u64,
    pub description: &'static str,
    pub cps_bonus_pct: f64,
    pub starter_cookies: f64,
    pub golden_freq_mult: f64,
}

pub const HEAVENLY_UPGRADES: &[HeavenlyUpgradeDef] = &[
    HeavenlyUpgradeDef {
        id: 0,
        name: "Legacy",
        cost_chips: 1,
        description:
            "Unlocks the potential of your Heavenly Chips (+1% CpS per unspent/total prestige).",
        cps_bonus_pct: 0.10,
        starter_cookies: 0.0,
        golden_freq_mult: 1.0,
    },
    HeavenlyUpgradeDef {
        id: 1,
        name: "Heavenly cookies",
        cost_chips: 3,
        description: "Cookie production multiplier +10% permanently.",
        cps_bonus_pct: 0.10,
        starter_cookies: 0.0,
        golden_freq_mult: 1.0,
    },
    HeavenlyUpgradeDef {
        id: 2,
        name: "Tin of british tea biscuits",
        cost_chips: 25,
        description: "Cookie production multiplier +15% permanently.",
        cps_bonus_pct: 0.15,
        starter_cookies: 0.0,
        golden_freq_mult: 1.0,
    },
    HeavenlyUpgradeDef {
        id: 3,
        name: "Starter kit",
        cost_chips: 50,
        description: "Start every ascension with 10,000 cookies and 10 Cursors.",
        cps_bonus_pct: 0.05,
        starter_cookies: 10_000.0,
        golden_freq_mult: 1.0,
    },
    HeavenlyUpgradeDef {
        id: 4,
        name: "Heavenly luck",
        cost_chips: 77,
        description: "Golden cookies appear 50% more often.",
        cps_bonus_pct: 0.05,
        starter_cookies: 0.0,
        golden_freq_mult: 1.5,
    },
    HeavenlyUpgradeDef {
        id: 5,
        name: "Permanent upgrade slot I",
        cost_chips: 100,
        description: "Boosts all building production by +25% permanently.",
        cps_bonus_pct: 0.25,
        starter_cookies: 100_000.0,
        golden_freq_mult: 1.0,
    },
    HeavenlyUpgradeDef {
        id: 6,
        name: "How to bake your dragon",
        cost_chips: 9,
        description: "Unlocks the Crumblor cookie dragon training.",
        cps_bonus_pct: 0.05,
        starter_cookies: 0.0,
        golden_freq_mult: 1.0,
    },
    HeavenlyUpgradeDef {
        id: 7,
        name: "Season switcher",
        cost_chips: 111,
        description: "Allows triggering festive seasons at will (+20% CpS).",
        cps_bonus_pct: 0.20,
        starter_cookies: 0.0,
        golden_freq_mult: 1.1,
    },
];

#[derive(Debug, Clone, Copy)]
pub enum AchievementUnlock {
    CookiesBaked(f64),
    CpsReached(f64),
    TotalClicks(u64),
    BuildingCount { building_id: usize, count: u32 },
    GoldenClicks(u64),
    WrinklersPopped(u64),
    Ascensions(u64),
    UpgradesPurchased(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct AchievementDef {
    pub id: usize,
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub unlock: AchievementUnlock,
}

pub const ACHIEVEMENTS: &[AchievementDef] = &[
    // Baking milestones
    AchievementDef {
        id: 0,
        name: "Wake and bake",
        description: "Bake 1 cookie in one ascension.",
        category: "Production",
        unlock: AchievementUnlock::CookiesBaked(1.0),
    },
    AchievementDef {
        id: 1,
        name: "Making some dough",
        description: "Bake 1,000 cookies in one ascension.",
        category: "Production",
        unlock: AchievementUnlock::CookiesBaked(1_000.0),
    },
    AchievementDef {
        id: 2,
        name: "So baked right now",
        description: "Bake 100,000 cookies in one ascension.",
        category: "Production",
        unlock: AchievementUnlock::CookiesBaked(100_000.0),
    },
    AchievementDef {
        id: 3,
        name: "Fledgling bakery",
        description: "Bake 1,000,000 cookies in one ascension.",
        category: "Production",
        unlock: AchievementUnlock::CookiesBaked(1_000_000.0),
    },
    AchievementDef {
        id: 4,
        name: "Affluent bakery",
        description: "Bake 100,000,000 cookies in one ascension.",
        category: "Production",
        unlock: AchievementUnlock::CookiesBaked(100_000_000.0),
    },
    AchievementDef {
        id: 5,
        name: "World-famous bakery",
        description: "Bake 1,000,000,000 cookies in one ascension.",
        category: "Production",
        unlock: AchievementUnlock::CookiesBaked(1_000_000_000.0),
    },
    AchievementDef {
        id: 6,
        name: "Cosmic bakery",
        description: "Bake 1,000,000,000,000 cookies in one ascension.",
        category: "Production",
        unlock: AchievementUnlock::CookiesBaked(1_000_000_000_000.0),
    },
    AchievementDef {
        id: 7,
        name: "Galactic bakery",
        description: "Bake 1e15 cookies in one ascension.",
        category: "Production",
        unlock: AchievementUnlock::CookiesBaked(1e15),
    },
    // CPS milestones
    AchievementDef {
        id: 8,
        name: "Casual baking",
        description: "Bake 1 cookie per second.",
        category: "CpS",
        unlock: AchievementUnlock::CpsReached(1.0),
    },
    AchievementDef {
        id: 9,
        name: "Hardcore baking",
        description: "Bake 10 cookies per second.",
        category: "CpS",
        unlock: AchievementUnlock::CpsReached(10.0),
    },
    AchievementDef {
        id: 10,
        name: "Steady tasty stream",
        description: "Bake 100 cookies per second.",
        category: "CpS",
        unlock: AchievementUnlock::CpsReached(100.0),
    },
    AchievementDef {
        id: 11,
        name: "Cookie monster",
        description: "Bake 1,000 cookies per second.",
        category: "CpS",
        unlock: AchievementUnlock::CpsReached(1_000.0),
    },
    AchievementDef {
        id: 12,
        name: "Mass producer",
        description: "Bake 10,000 cookies per second.",
        category: "CpS",
        unlock: AchievementUnlock::CpsReached(10_000.0),
    },
    AchievementDef {
        id: 13,
        name: "Cookie vortex",
        description: "Bake 1,000,000 cookies per second.",
        category: "CpS",
        unlock: AchievementUnlock::CpsReached(1_000_000.0),
    },
    AchievementDef {
        id: 14,
        name: "Cookie pulsar",
        description: "Bake 1,000,000,000 cookies per second.",
        category: "CpS",
        unlock: AchievementUnlock::CpsReached(1_000_000_000.0),
    },
    // Clicking milestones
    AchievementDef {
        id: 15,
        name: "Clicktastic",
        description: "Click the big cookie 10 times.",
        category: "Clicking",
        unlock: AchievementUnlock::TotalClicks(10),
    },
    AchievementDef {
        id: 16,
        name: "Clickathlon",
        description: "Click the big cookie 100 times.",
        category: "Clicking",
        unlock: AchievementUnlock::TotalClicks(100),
    },
    AchievementDef {
        id: 17,
        name: "Clickolympics",
        description: "Click the big cookie 1,000 times.",
        category: "Clicking",
        unlock: AchievementUnlock::TotalClicks(1_000),
    },
    AchievementDef {
        id: 18,
        name: "Clickorama",
        description: "Click the big cookie 5,000 times.",
        category: "Clicking",
        unlock: AchievementUnlock::TotalClicks(5_000),
    },
    // Building milestones (first of each + stacks)
    AchievementDef {
        id: 19,
        name: "Click",
        description: "Have 1 cursor.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 0,
            count: 1,
        },
    },
    AchievementDef {
        id: 20,
        name: "Double-click",
        description: "Have 2 cursors.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 0,
            count: 2,
        },
    },
    AchievementDef {
        id: 21,
        name: "Mouse wheel",
        description: "Have 50 cursors.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 0,
            count: 50,
        },
    },
    AchievementDef {
        id: 22,
        name: "Grandma's cookies",
        description: "Have 1 grandma.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 1,
            count: 1,
        },
    },
    AchievementDef {
        id: 23,
        name: "Sloppy kisses",
        description: "Have 50 grandmas.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 1,
            count: 50,
        },
    },
    AchievementDef {
        id: 24,
        name: "My first farm",
        description: "Have 1 farm.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 2,
            count: 1,
        },
    },
    AchievementDef {
        id: 25,
        name: "Reap what you sow",
        description: "Have 50 farms.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 2,
            count: 50,
        },
    },
    AchievementDef {
        id: 26,
        name: "You know the drill",
        description: "Have 1 mine.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 3,
            count: 1,
        },
    },
    AchievementDef {
        id: 27,
        name: "Production chain",
        description: "Have 1 factory.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 4,
            count: 1,
        },
    },
    AchievementDef {
        id: 28,
        name: "Pretty penny",
        description: "Have 1 bank.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 5,
            count: 1,
        },
    },
    AchievementDef {
        id: 29,
        name: "Your time to shrine",
        description: "Have 1 temple.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 6,
            count: 1,
        },
    },
    AchievementDef {
        id: 30,
        name: "Bewitched",
        description: "Have 1 wizard tower.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 7,
            count: 1,
        },
    },
    AchievementDef {
        id: 31,
        name: "Expedition",
        description: "Have 1 shipment.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 8,
            count: 1,
        },
    },
    AchievementDef {
        id: 32,
        name: "Transmutation",
        description: "Have 1 alchemy lab.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 9,
            count: 1,
        },
    },
    AchievementDef {
        id: 33,
        name: "A whole new world",
        description: "Have 1 portal.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 10,
            count: 1,
        },
    },
    AchievementDef {
        id: 34,
        name: "Time warp",
        description: "Have 1 time machine.",
        category: "Buildings",
        unlock: AchievementUnlock::BuildingCount {
            building_id: 11,
            count: 1,
        },
    },
    // Special / Golden / Wrinklers / Prestige
    AchievementDef {
        id: 35,
        name: "Golden cookie",
        description: "Click 1 golden cookie.",
        category: "Special",
        unlock: AchievementUnlock::GoldenClicks(1),
    },
    AchievementDef {
        id: 36,
        name: "Lucky cookie",
        description: "Click 7 golden cookies.",
        category: "Special",
        unlock: AchievementUnlock::GoldenClicks(7),
    },
    AchievementDef {
        id: 37,
        name: "Fortune",
        description: "Click 27 golden cookies.",
        category: "Special",
        unlock: AchievementUnlock::GoldenClicks(27),
    },
    AchievementDef {
        id: 38,
        name: "Itchscratcher",
        description: "Burst 1 wrinkler.",
        category: "Special",
        unlock: AchievementUnlock::WrinklersPopped(1),
    },
    AchievementDef {
        id: 39,
        name: "Wrinklesquisher",
        description: "Burst 50 wrinklers.",
        category: "Special",
        unlock: AchievementUnlock::WrinklersPopped(50),
    },
    AchievementDef {
        id: 40,
        name: "Rebirth",
        description: "Ascend at least once.",
        category: "Prestige",
        unlock: AchievementUnlock::Ascensions(1),
    },
    AchievementDef {
        id: 41,
        name: "Enhancer",
        description: "Purchase 5 upgrades.",
        category: "Special",
        unlock: AchievementUnlock::UpgradesPurchased(5),
    },
    AchievementDef {
        id: 42,
        name: "Augmenter",
        description: "Purchase 15 upgrades.",
        category: "Special",
        unlock: AchievementUnlock::UpgradesPurchased(15),
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum Season {
    #[default]
    None,
    Christmas,
    Valentines,
    Halloween,
    Easter,
    BusinessDay,
}

impl Season {
    pub const ALL: [Season; 6] = [
        Season::None,
        Season::Christmas,
        Season::Valentines,
        Season::Halloween,
        Season::Easter,
        Season::BusinessDay,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Season::None => "No Season",
            Season::Christmas => "Christmas (Festive)",
            Season::Valentines => "Valentine's Day",
            Season::Halloween => "Halloween",
            Season::Easter => "Easter",
            Season::BusinessDay => "Business Day",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Season::None => "Standard baking weather.",
            Season::Christmas => "Reindeer shimmers & +5% festive CpS.",
            Season::Valentines => "Heart biscuits & +5% CpS.",
            Season::Halloween => "Spooky wrinkler drops & +5% CpS.",
            Season::Easter => "Golden eggs & +8% CpS.",
            Season::BusinessDay => "Golden cookies spawn 5% more often & +5% CpS.",
        }
    }

    pub fn cps_multiplier(self) -> f64 {
        match self {
            Season::None => 1.0,
            Season::Easter => 1.08,
            _ => 1.05,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum DragonAura {
    #[default]
    None,
    BreathOfMilk,
    DragonCursor,
    ElderBattalion,
    RadiantAppetite,
}

impl DragonAura {
    pub const ALL: [DragonAura; 5] = [
        DragonAura::None,
        DragonAura::BreathOfMilk,
        DragonAura::DragonCursor,
        DragonAura::ElderBattalion,
        DragonAura::RadiantAppetite,
    ];

    pub fn name(self) -> &'static str {
        match self {
            DragonAura::None => "No Aura",
            DragonAura::BreathOfMilk => "Breath of Milk",
            DragonAura::DragonCursor => "Dragon Cursor",
            DragonAura::ElderBattalion => "Elder Battalion",
            DragonAura::RadiantAppetite => "Radiant Appetite",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            DragonAura::None => "No active dragon aura.",
            DragonAura::BreathOfMilk => "Kittens are 5% more effective.",
            DragonAura::DragonCursor => "Clicking is 5% more powerful.",
            DragonAura::ElderBattalion => "Grandmas gain +1% CpS for every non-grandma building.",
            DragonAura::RadiantAppetite => "All cookie production is multiplied by 2x!",
        }
    }

    pub fn required_dragon_level(self) -> u8 {
        match self {
            DragonAura::None => 0,
            DragonAura::BreathOfMilk => 1,
            DragonAura::DragonCursor => 2,
            DragonAura::ElderBattalion => 3,
            DragonAura::RadiantAppetite => 4,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GardenSeedDef {
    pub id: usize,
    pub name: &'static str,
    pub mature_ticks: u32,
    pub cps_bonus_pct: f64,
    pub click_bonus_pct: f64,
}

pub const GARDEN_SEEDS: [GardenSeedDef; 4] = [
    GardenSeedDef {
        id: 0,
        name: "Baker's Wheat",
        mature_ticks: 10,
        cps_bonus_pct: 0.01,
        click_bonus_pct: 0.0,
    },
    GardenSeedDef {
        id: 1,
        name: "Thumbcorn",
        mature_ticks: 12,
        cps_bonus_pct: 0.0,
        click_bonus_pct: 0.02,
    },
    GardenSeedDef {
        id: 2,
        name: "Golden Clover",
        mature_ticks: 18,
        cps_bonus_pct: 0.03,
        click_bonus_pct: 0.0,
    },
    GardenSeedDef {
        id: 3,
        name: "Whiskerbloom",
        mature_ticks: 24,
        cps_bonus_pct: 0.04,
        click_bonus_pct: 0.01,
    },
];

#[derive(Debug, Clone, Copy)]
pub struct SpellDef {
    pub id: usize,
    pub name: &'static str,
    pub magic_cost: f64,
    pub description: &'static str,
}

pub const GRIMOIRE_SPELLS: [SpellDef; 3] = [
    SpellDef {
        id: 0,
        name: "Conjure Baked Goods",
        magic_cost: 15.0,
        description: "Summon 30 minutes of CpS instantly (up to 15% of bank).",
    },
    SpellDef {
        id: 1,
        name: "Force the Hand of Fate",
        magic_cost: 25.0,
        description: "Summon a Golden Cookie immediately.",
    },
    SpellDef {
        id: 2,
        name: "Stretch Time",
        magic_cost: 20.0,
        description: "Extends all active buffs by 20%.",
    },
];

#[derive(Debug, Clone, Copy)]
pub struct PantheonSpiritDef {
    pub id: usize,
    pub name: &'static str,
    pub description: &'static str,
    pub cps_mult: f64,
    pub click_mult: f64,
}

pub const PANTHEON_SPIRITS: [PantheonSpiritDef; 4] = [
    PantheonSpiritDef {
        id: 0,
        name: "None",
        description: "Slot empty.",
        cps_mult: 1.0,
        click_mult: 1.0,
    },
    PantheonSpiritDef {
        id: 1,
        name: "Mokalsium, Mother Spirit",
        description: "Milk is more powerful (+10% CpS).",
        cps_mult: 1.10,
        click_mult: 0.98,
    },
    PantheonSpiritDef {
        id: 2,
        name: "Muridal, Spirit of Labor",
        description: "Clicks are +15% stronger; buildings -2% CpS.",
        cps_mult: 0.98,
        click_mult: 1.15,
    },
    PantheonSpiritDef {
        id: 3,
        name: "Jeremy, Spirit of Industry",
        description: "Buildings produce +8% more cookies.",
        cps_mult: 1.08,
        click_mult: 1.0,
    },
];

pub const STOCK_SYMBOLS: [&str; 6] = ["CRL", "CHC", "BTR", "SUG", "NUT", "CKI"];

pub const NEWS_EARLY: &[&str] = &[
    "You feel like making cookies. But nobody wants to eat your cookies.",
    "Your first batch goes in the trash. The neighborhood raccoon barely touches it.",
    "Your family accepts to try some of your cookies.",
    "Your cookies are popular in the neighborhood.",
    "People are starting to talk about your cookies.",
];

pub const NEWS_MID: &[&str] = &[
    "Your cookies are talked about for miles around.",
    "Your cookies are renowned across the whole country!",
    "New cookie museum opens in the capital; admission payable in chocolate chips.",
    "Scientists warn that cookie-based economy may lead to 'unprecedented deliciousness'.",
    "Local grandmas unionize, demanding warmer knitting needles and extra dough.",
    "Cookie factories report record output as terminal operators mash spacebars worldwide.",
];

pub const NEWS_LATE: &[&str] = &[
    "Intergalactic freighters queue in low orbit to offload fresh space cookies.",
    "Physicists confirm the universe is 73% dark cookie dough by mass.",
    "Time travelers warn against eating cookies before they are baked, then do it anyway.",
    "Prisms refract pure sunlight into warm, chewy snickerdoodles.",
    "Multiverse counsel declares your bakery a Type-IV Kardashev civilization.",
];

pub const NEWS_GRANDMAPOCALYPSE: &[&str] = &[
    "News : moist wrinklers spotted clinging to the terminal border!",
    "News : grandmas heard chanting in an ancient, buttery dialect.",
    "News : 'We are one dough, one mind,' says local grandmother of twelve.",
    "News : citizens advised not to make eye contact with red wrath cookies.",
];
