use crate::data::{
    AchievementUnlock, DragonAura, Season, UpgradeEffect, UpgradeUnlock, ACHIEVEMENTS, BUILDINGS,
    GARDEN_SEEDS, GRIMOIRE_SPELLS, HEAVENLY_UPGRADES, NEWS_EARLY, NEWS_GRANDMAPOCALYPSE, NEWS_LATE,
    NEWS_MID, NUM_BUILDINGS, PANTHEON_SPIRITS, STOCK_SYMBOLS, UPGRADES,
};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

pub const MAX_WRINKLERS: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BuyAmount {
    #[default]
    One,
    Ten,
    Hundred,
    Max,
}

impl BuyAmount {
    pub fn next(self) -> Self {
        match self {
            BuyAmount::One => BuyAmount::Ten,
            BuyAmount::Ten => BuyAmount::Hundred,
            BuyAmount::Hundred => BuyAmount::Max,
            BuyAmount::Max => BuyAmount::One,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            BuyAmount::One => "x1",
            BuyAmount::Ten => "x10",
            BuyAmount::Hundred => "x100",
            BuyAmount::Max => "Max",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StoreMode {
    #[default]
    Buy,
    Sell,
}

impl StoreMode {
    pub fn toggle(self) -> Self {
        match self {
            StoreMode::Buy => StoreMode::Sell,
            StoreMode::Sell => StoreMode::Buy,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            StoreMode::Buy => "BUY",
            StoreMode::Sell => "SELL",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuffKind {
    Frenzy,
    ClickFrenzy,
    ElderFrenzy,
    Clot,
}

impl BuffKind {
    pub fn name(self) -> &'static str {
        match self {
            BuffKind::Frenzy => "Frenzy (x7 CpS)",
            BuffKind::ClickFrenzy => "Click Frenzy (x777 Click)",
            BuffKind::ElderFrenzy => "Elder Frenzy (x666 CpS)",
            BuffKind::Clot => "Clot (x0.5 CpS)",
        }
    }

    pub fn cps_multiplier(self) -> f64 {
        match self {
            BuffKind::Frenzy => 7.0,
            BuffKind::ClickFrenzy => 1.0,
            BuffKind::ElderFrenzy => 666.0,
            BuffKind::Clot => 0.5,
        }
    }

    pub fn click_multiplier(self) -> f64 {
        match self {
            BuffKind::ClickFrenzy => 777.0,
            _ => 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveBuff {
    pub kind: BuffKind,
    pub time_left: f64,
    pub max_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveShimmer {
    pub is_wrath: bool,
    pub time_left: f64,
    pub max_time: f64,
    pub x_norm: f32,
    pub y_norm: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Wrinkler {
    pub active: bool,
    pub cookies_sucked: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ClickCookie,
    BuyOrSellBuilding(usize),
    BuyBuilding(usize, BuyAmount),
    SellBuilding(usize, BuyAmount),
    BuyUpgrade(usize),
    BuyNextAvailableUpgrade,
    SetBuyAmount(BuyAmount),
    CycleBuyAmount,
    ToggleStoreMode,
    ClickShimmer,
    PopWrinkler(usize),
    PopAllWrinklers,
    Ascend,
    BuyHeavenlyUpgrade(usize),
    TrainDragon,
    CycleDragonAura,
    SetSeason(Season),
    CycleSeason,
    LevelUpBuilding(usize),
    GardenSelectSeed(usize),
    GardenInteractPlot(usize, usize),
    CastSpell(usize),
    BuyStock(usize),
    SellStock(usize),
    CyclePantheonSpirit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GameState {
    pub bakery_name: String,
    pub cookies: f64,
    pub cookies_baked_this_run: f64,
    pub cookies_baked_all_time: f64,
    pub total_clicks: u64,
    pub handmade_cookies: f64,
    pub golden_clicks: u64,
    pub wrinklers_popped: u64,
    pub ascensions: u64,
    pub run_time_secs: f64,
    pub total_time_secs: f64,

    pub buildings: [u32; NUM_BUILDINGS],
    pub building_levels: [u32; NUM_BUILDINGS],
    pub upgrades_purchased: Vec<bool>,
    pub achievements_unlocked: Vec<bool>,
    pub heavenly_upgrades_purchased: Vec<bool>,

    pub buy_amount: BuyAmount,
    pub store_mode: StoreMode,

    pub prestige_level: u64,
    pub heavenly_chips: u64,

    pub sugar_lumps: u32,
    pub sugar_lump_timer: f64,

    pub grandmapocalypse_stage: u8,
    pub elder_pledge_timer: f64,
    pub wrinklers: [Wrinkler; MAX_WRINKLERS],
    pub wrinkler_spawn_timer: f64,

    pub active_shimmer: Option<ActiveShimmer>,
    pub shimmer_spawn_timer: f64,
    pub active_buffs: Vec<ActiveBuff>,

    pub dragon_level: u8,
    pub dragon_aura: DragonAura,
    pub season: Season,
    pub season_timer: f64,

    // Minigames state
    pub garden_plots: [[Option<(usize, u32)>; 4]; 4],
    pub garden_selected_seed: usize,
    pub garden_tick_timer: f64,

    pub magic: f64,
    pub max_magic: f64,

    pub stock_prices: [f64; 6],
    pub stock_owned: [u32; 6],
    pub stock_tick_timer: f64,

    pub pantheon_spirit: usize,

    pub current_news: String,
    pub news_timer: f64,
    pub last_event_message: String,

    #[serde(skip)]
    pub newly_unlocked_achievements: Vec<usize>,
    #[serde(skip)]
    pub last_click_gain: f64,
    #[serde(skip)]
    rng: SmallRng,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            bakery_name: "Terminal".to_string(),
            cookies: 0.0,
            cookies_baked_this_run: 0.0,
            cookies_baked_all_time: 0.0,
            total_clicks: 0,
            handmade_cookies: 0.0,
            golden_clicks: 0,
            wrinklers_popped: 0,
            ascensions: 0,
            run_time_secs: 0.0,
            total_time_secs: 0.0,
            buildings: [0; NUM_BUILDINGS],
            building_levels: [0; NUM_BUILDINGS],
            upgrades_purchased: vec![false; UPGRADES.len()],
            achievements_unlocked: vec![false; ACHIEVEMENTS.len()],
            heavenly_upgrades_purchased: vec![false; HEAVENLY_UPGRADES.len()],
            buy_amount: BuyAmount::One,
            store_mode: StoreMode::Buy,
            prestige_level: 0,
            heavenly_chips: 0,
            sugar_lumps: 1,
            sugar_lump_timer: 0.0,
            grandmapocalypse_stage: 0,
            elder_pledge_timer: 0.0,
            wrinklers: [Wrinkler::default(); MAX_WRINKLERS],
            wrinkler_spawn_timer: 25.0,
            active_shimmer: None,
            shimmer_spawn_timer: 45.0,
            active_buffs: Vec::new(),
            dragon_level: 0,
            dragon_aura: DragonAura::None,
            season: Season::None,
            season_timer: 0.0,
            garden_plots: [[None; 4]; 4],
            garden_selected_seed: 0,
            garden_tick_timer: 0.0,
            magic: 15.0,
            max_magic: 15.0,
            stock_prices: [10.0, 25.0, 40.0, 60.0, 95.0, 150.0],
            stock_owned: [0; 6],
            stock_tick_timer: 0.0,
            pantheon_spirit: 0,
            current_news: NEWS_EARLY[0].to_string(),
            news_timer: 10.0,
            last_event_message: "Welcome to Cookie-CLI! Press SPACE or ENTER to bake.".to_string(),
            newly_unlocked_achievements: Vec::new(),
            last_click_gain: 1.0,
            rng: SmallRng::from_entropy(),
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(seed),
            ..Self::default()
        }
    }

    /// Ensures vector lengths match static definitions after loading an older save.
    pub fn normalize_after_load(&mut self) {
        if self.upgrades_purchased.len() < UPGRADES.len() {
            self.upgrades_purchased.resize(UPGRADES.len(), false);
        }
        if self.achievements_unlocked.len() < ACHIEVEMENTS.len() {
            self.achievements_unlocked.resize(ACHIEVEMENTS.len(), false);
        }
        if self.heavenly_upgrades_purchased.len() < HEAVENLY_UPGRADES.len() {
            self.heavenly_upgrades_purchased
                .resize(HEAVENLY_UPGRADES.len(), false);
        }
        if !self.cookies.is_finite() || self.cookies < 0.0 {
            self.cookies = 0.0;
        }
        if !self.cookies_baked_this_run.is_finite() || self.cookies_baked_this_run < 0.0 {
            self.cookies_baked_this_run = self.cookies;
        }
        if !self.cookies_baked_all_time.is_finite()
            || self.cookies_baked_all_time < self.cookies_baked_this_run
        {
            self.cookies_baked_all_time = self.cookies_baked_this_run;
        }
    }

    /// Total buildings owned across all 20 types.
    pub fn total_buildings_owned(&self) -> u32 {
        self.buildings.iter().copied().sum()
    }

    /// Cost of buying the single `(owned + 1)`-th building of `building_id`.
    pub fn single_building_cost(building_id: usize, owned: u32) -> f64 {
        let base = BUILDINGS[building_id].base_cost;
        (base * 1.15_f64.powi(owned as i32)).ceil()
    }

    /// Cost of buying `count` buildings starting from `owned`.
    pub fn bulk_buy_cost(building_id: usize, owned: u32, count: u32) -> f64 {
        let mut total = 0.0;
        for i in 0..count {
            total += Self::single_building_cost(building_id, owned.saturating_add(i));
        }
        total
    }

    /// Computes `(quantity, cost_or_refund)` for the given `StoreMode` and `BuyAmount`.
    /// When `BuyAmount::Max` is selected in `Buy` mode and the player cannot afford even 1,
    /// returns `(1, single_cost)` so the UI displays the next building price.
    pub fn store_quote(
        &self,
        building_id: usize,
        mode: StoreMode,
        amount: BuyAmount,
    ) -> (u32, f64) {
        let owned = self.buildings[building_id];
        match mode {
            StoreMode::Buy => match amount {
                BuyAmount::One => (1, Self::single_building_cost(building_id, owned)),
                BuyAmount::Ten => (10, Self::bulk_buy_cost(building_id, owned, 10)),
                BuyAmount::Hundred => (100, Self::bulk_buy_cost(building_id, owned, 100)),
                BuyAmount::Max => {
                    let first_cost = Self::single_building_cost(building_id, owned);
                    if self.cookies < first_cost {
                        return (1, first_cost);
                    }
                    let mut count = 0_u32;
                    let mut total = 0.0;
                    while count < 1_000 {
                        let next_c = Self::single_building_cost(building_id, owned + count);
                        if total + next_c > self.cookies {
                            break;
                        }
                        total += next_c;
                        count += 1;
                    }
                    (count.max(1), total)
                }
            },
            StoreMode::Sell => {
                if owned == 0 {
                    return (0, 0.0);
                }
                let target = match amount {
                    BuyAmount::One => 1,
                    BuyAmount::Ten => 10,
                    BuyAmount::Hundred => 100,
                    BuyAmount::Max => owned,
                };
                let sell_count = target.min(owned);
                let mut refund = 0.0;
                for i in 1..=sell_count {
                    refund += (Self::single_building_cost(building_id, owned - i) * 0.5).floor();
                }
                (sell_count, refund)
            }
        }
    }

    /// Number of unlocked achievements.
    pub fn unlocked_achievements_count(&self) -> usize {
        self.achievements_unlocked.iter().filter(|&&u| u).count()
    }

    /// Number of purchased upgrades.
    pub fn purchased_upgrades_count(&self) -> usize {
        self.upgrades_purchased.iter().filter(|&&u| u).count()
    }

    /// Milk progress (0.0 to ~2.0+), +4% per achievement.
    pub fn milk_progress(&self) -> f64 {
        self.unlocked_achievements_count() as f64 * 0.04
    }

    /// Milk flavor label based on achievement progress.
    pub fn milk_name(&self) -> &'static str {
        let m = self.milk_progress();
        if m >= 1.2 {
            "Cosmic Caramel Milk"
        } else if m >= 0.8 {
            "Raspberry Milk"
        } else if m >= 0.4 {
            "Chocolate Milk"
        } else {
            "Plain Milk"
        }
    }

    /// Calculates the Thousand Fingers flat bonus per non-cursor building.
    pub fn thousand_fingers_bonus(&self) -> f64 {
        let non_cursor_count: u32 = self.buildings.iter().skip(1).copied().sum();
        if non_cursor_count == 0 {
            return 0.0;
        }
        let mut per_building = 0.0;
        for (i, upg) in UPGRADES.iter().enumerate() {
            if self.upgrades_purchased.get(i).copied().unwrap_or(false) {
                if let UpgradeEffect::ThousandFingers {
                    bonus_per_non_cursor,
                } = upg.effect
                {
                    per_building += bonus_per_non_cursor;
                }
            }
        }
        per_building * (non_cursor_count as f64)
    }

    /// Computes the CPS of a single unit of `building_id`, taking building upgrades and synergies into account.
    pub fn single_building_cps(&self, building_id: usize) -> f64 {
        let mut cps = BUILDINGS[building_id].base_cps;
        let mut mult = 1.0;

        for (i, upg) in UPGRADES.iter().enumerate() {
            if !self.upgrades_purchased.get(i).copied().unwrap_or(false) {
                continue;
            }
            match upg.effect {
                UpgradeEffect::BuildingMultiplier {
                    building_id: bid,
                    factor,
                } if bid == building_id => {
                    mult *= factor;
                }
                UpgradeEffect::GrandmaSynergy { target_building } => {
                    if building_id == 1 {
                        mult *= 2.0;
                    } else if building_id == target_building {
                        mult *= 1.0 + 0.01 * (self.buildings[1] as f64);
                    }
                }
                UpgradeEffect::Synergy {
                    building_a,
                    building_b,
                } => {
                    if building_id == building_a {
                        mult *= 1.0 + 0.05 * (self.buildings[building_b] as f64);
                    } else if building_id == building_b {
                        mult *= 1.0 + 0.001 * (self.buildings[building_a] as f64);
                    }
                }
                _ => {}
            }
        }

        // Sugar lump building level bonus (+1% per level)
        let lvl = self.building_levels[building_id];
        if lvl > 0 {
            mult *= 1.0 + 0.01 * (lvl as f64);
        }

        // Dragon aura: Elder Battalion
        if building_id == 1 && self.dragon_aura == DragonAura::ElderBattalion {
            let non_grandmas: u32 = self
                .buildings
                .iter()
                .enumerate()
                .filter(|&(idx, _)| idx != 1)
                .map(|(_, &c)| c)
                .sum();
            mult *= 1.0 + 0.01 * (non_grandmas as f64);
        }

        // Grandmapocalypse stage bonus to Grandmas
        if building_id == 1 && self.effective_grandmapocalypse_stage() > 0 {
            cps += 0.02 * (self.effective_grandmapocalypse_stage() as f64);
        }

        cps *= mult;

        if building_id == 0 {
            cps += self.thousand_fingers_bonus();
        }

        cps
    }

    /// Global CPS multiplier from upgrades, kittens, prestige, buffs, dragon, season, garden, and pantheon.
    pub fn global_cps_multiplier(&self) -> f64 {
        let mut mult = 1.0;

        let milk = self.milk_progress();
        let milk_aura_boost = if self.dragon_aura == DragonAura::BreathOfMilk {
            1.05
        } else {
            1.0
        };

        for (i, upg) in UPGRADES.iter().enumerate() {
            if !self.upgrades_purchased.get(i).copied().unwrap_or(false) {
                continue;
            }
            match upg.effect {
                UpgradeEffect::GlobalCpsMultiplier { factor } => {
                    mult *= factor;
                }
                UpgradeEffect::Kitten { milk_factor } => {
                    mult *= 1.0 + milk * milk_factor * milk_aura_boost;
                }
                _ => {}
            }
        }

        // Prestige bonus (+1% per prestige level once Legacy or any prestige is active)
        if self.prestige_level > 0 {
            let legacy_factor = if self
                .heavenly_upgrades_purchased
                .first()
                .copied()
                .unwrap_or(false)
            {
                1.0
            } else {
                0.5
            };
            mult *= 1.0 + (self.prestige_level as f64) * 0.01 * legacy_factor;
        }

        for (i, hupg) in HEAVENLY_UPGRADES.iter().enumerate() {
            if self
                .heavenly_upgrades_purchased
                .get(i)
                .copied()
                .unwrap_or(false)
            {
                mult *= 1.0 + hupg.cps_bonus_pct;
            }
        }

        // Season multiplier
        mult *= self.season.cps_multiplier();

        // Dragon aura: Radiant Appetite
        if self.dragon_aura == DragonAura::RadiantAppetite {
            mult *= 2.0;
        }

        // Garden mature plants bonus
        let mut garden_cps_bonus = 0.0;
        for row in &self.garden_plots {
            for cell in row.iter().flatten() {
                let (seed_id, age) = *cell;
                if let Some(seed) = GARDEN_SEEDS.get(seed_id) {
                    if age >= seed.mature_ticks {
                        garden_cps_bonus += seed.cps_bonus_pct;
                    }
                }
            }
        }
        mult *= 1.0 + garden_cps_bonus;

        // Pantheon spirit multiplier
        if let Some(spirit) = PANTHEON_SPIRITS.get(self.pantheon_spirit) {
            mult *= spirit.cps_mult;
        }

        // Active buffs
        for buff in &self.active_buffs {
            mult *= buff.kind.cps_multiplier();
        }

        mult
    }

    /// Raw un-leeched CPS across all buildings.
    pub fn raw_cps(&self) -> f64 {
        let mut base_sum = 0.0;
        for id in 0..NUM_BUILDINGS {
            let count = self.buildings[id];
            if count > 0 {
                base_sum += (count as f64) * self.single_building_cps(id);
            }
        }
        base_sum * self.global_cps_multiplier()
    }

    /// Number of currently active wrinklers.
    pub fn active_wrinkler_count(&self) -> usize {
        self.wrinklers.iter().filter(|w| w.active).count()
    }

    /// Fraction of CPS currently leeched by wrinklers (5% per wrinkler, up to 50%).
    pub fn wrinkler_leech_fraction(&self) -> f64 {
        (self.active_wrinkler_count() as f64 * 0.05).min(0.50)
    }

    /// Net CPS added directly to the cookie bank after wrinkler leeching.
    pub fn effective_cps(&self) -> f64 {
        self.raw_cps() * (1.0 - self.wrinkler_leech_fraction())
    }

    /// Cookies gained per manual click (CPC).
    pub fn cookies_per_click(&self) -> f64 {
        let mut click_base = 1.0;

        // Cursor multiplier upgrades also double base click power
        for (i, upg) in UPGRADES.iter().enumerate() {
            if !self.upgrades_purchased.get(i).copied().unwrap_or(false) {
                continue;
            }
            if let UpgradeEffect::BuildingMultiplier {
                building_id: 0,
                factor,
            } = upg.effect
            {
                click_base *= factor;
            }
        }

        click_base += self.thousand_fingers_bonus();

        // Percentage of CPS added to clicks from mouse upgrades
        let mut cps_pct = 0.0;
        for (i, upg) in UPGRADES.iter().enumerate() {
            if !self.upgrades_purchased.get(i).copied().unwrap_or(false) {
                continue;
            }
            if let UpgradeEffect::ClickCpsPercent { pct } = upg.effect {
                cps_pct += pct;
            }
        }

        let mut total = click_base + self.raw_cps() * cps_pct;

        if self.dragon_aura == DragonAura::DragonCursor {
            total *= 1.05;
        }

        // Garden click bonus
        let mut garden_click_bonus = 0.0;
        for row in &self.garden_plots {
            for cell in row.iter().flatten() {
                let (seed_id, age) = *cell;
                if let Some(seed) = GARDEN_SEEDS.get(seed_id) {
                    if age >= seed.mature_ticks {
                        garden_click_bonus += seed.click_bonus_pct;
                    }
                }
            }
        }
        total *= 1.0 + garden_click_bonus;

        if let Some(spirit) = PANTHEON_SPIRITS.get(self.pantheon_spirit) {
            total *= spirit.click_mult;
        }

        for buff in &self.active_buffs {
            total *= buff.kind.click_multiplier();
        }

        total.max(1.0)
    }

    /// Effective Grandmapocalypse stage (0 if Elder Pledge is active).
    pub fn effective_grandmapocalypse_stage(&self) -> u8 {
        if self.elder_pledge_timer > 0.0 {
            0
        } else {
            self.grandmapocalypse_stage
        }
    }

    pub fn grandmapocalypse_status_name(&self) -> &'static str {
        if self.elder_pledge_timer > 0.0 {
            return "Appeased (Elder Pledge)";
        }
        match self.grandmapocalypse_stage {
            0 => "Appeased",
            1 => "Stage 1: Awoken",
            2 => "Stage 2: Displeased",
            _ => "Stage 3: Angered",
        }
    }

    /// Checks if an upgrade's unlock condition is met.
    pub fn is_upgrade_unlocked(&self, upgrade_id: usize) -> bool {
        let Some(upg) = UPGRADES.get(upgrade_id) else {
            return false;
        };
        match upg.unlock {
            UpgradeUnlock::BuildingCount { building_id, count } => {
                self.buildings[building_id] >= count
            }
            UpgradeUnlock::TotalClicks(clicks) => self.total_clicks >= clicks,
            UpgradeUnlock::CookiesBaked(amount) => self.cookies_baked_this_run >= amount,
            UpgradeUnlock::AchievementCount(cnt) => self.unlocked_achievements_count() >= cnt,
            UpgradeUnlock::GrandmapocalypseStage(stage) => self.grandmapocalypse_stage >= stage,
        }
    }

    /// Returns indices of all unlocked, unpurchased upgrades sorted by cost ascending.
    pub fn available_upgrades(&self) -> Vec<usize> {
        let mut list: Vec<usize> = (0..UPGRADES.len())
            .filter(|&id| {
                !self.upgrades_purchased.get(id).copied().unwrap_or(false)
                    && self.is_upgrade_unlocked(id)
            })
            .collect();
        list.sort_by(|&a, &b| {
            UPGRADES[a]
                .cost
                .partial_cmp(&UPGRADES[b].cost)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        list
    }

    /// Prestige level corresponding to all-time baked cookies: `floor(cbrt(all_time / 1e12))`.
    pub fn total_prestige_from_cookies(cookies_all_time: f64) -> u64 {
        if cookies_all_time < 1_000_000_000_000.0 {
            return 0;
        }
        (cookies_all_time / 1_000_000_000_000.0).cbrt().floor() as u64
    }

    /// Additional prestige levels / Heavenly Chips that will be gained on ascending right now.
    pub fn pending_prestige(&self) -> u64 {
        Self::total_prestige_from_cookies(self.cookies_baked_all_time)
            .saturating_sub(self.prestige_level)
    }

    /// Bake cookies and add them to current bank, run total, and all-time total.
    pub fn bake_cookies(&mut self, amount: f64) {
        if !amount.is_finite() || amount <= 0.0 {
            return;
        }
        self.cookies += amount;
        self.cookies_baked_this_run += amount;
        self.cookies_baked_all_time += amount;
    }

    /// Evaluates all achievement conditions and records newly unlocked achievements.
    pub fn check_achievements(&mut self) {
        let cps = self.raw_cps();
        let upgrades_cnt = self.purchased_upgrades_count();

        for (id, ach) in ACHIEVEMENTS.iter().enumerate() {
            if self.achievements_unlocked.get(id).copied().unwrap_or(false) {
                continue;
            }
            let unlocked = match ach.unlock {
                AchievementUnlock::CookiesBaked(req) => self.cookies_baked_this_run >= req,
                AchievementUnlock::CpsReached(req) => cps >= req,
                AchievementUnlock::TotalClicks(req) => self.total_clicks >= req,
                AchievementUnlock::BuildingCount { building_id, count } => {
                    self.buildings[building_id] >= count
                }
                AchievementUnlock::GoldenClicks(req) => self.golden_clicks >= req,
                AchievementUnlock::WrinklersPopped(req) => self.wrinklers_popped >= req,
                AchievementUnlock::Ascensions(req) => self.ascensions >= req,
                AchievementUnlock::UpgradesPurchased(req) => upgrades_cnt >= req,
            };
            if unlocked {
                if id < self.achievements_unlocked.len() {
                    self.achievements_unlocked[id] = true;
                }
                self.newly_unlocked_achievements.push(id);
                self.last_event_message = format!("Achievement Unlocked: {}!", ach.name);
            }
        }
    }

    /// Advances the simulation by `dt` seconds.
    pub fn tick(&mut self, dt: f64) {
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }

        self.run_time_secs += dt;
        self.total_time_secs += dt;

        // 1. Cookie production & Wrinkler leeching
        let raw = self.raw_cps();
        let active_wrinklers = self.active_wrinkler_count();
        let leech_frac = (active_wrinklers as f64 * 0.05).min(0.50);
        let net_cps = raw * (1.0 - leech_frac);

        self.bake_cookies(net_cps * dt);

        if active_wrinklers > 0 {
            let per_wrinkler_rate = raw * 0.05 * dt;
            for w in &mut self.wrinklers {
                if w.active {
                    w.cookies_sucked += per_wrinkler_rate;
                }
            }
        }

        // 2. Active buffs countdown
        for buff in &mut self.active_buffs {
            buff.time_left -= dt;
        }
        self.active_buffs.retain(|b| b.time_left > 0.0);

        // 3. Elder Pledge & Season timers
        if self.elder_pledge_timer > 0.0 {
            self.elder_pledge_timer = (self.elder_pledge_timer - dt).max(0.0);
        }
        if self.season != Season::None && self.season_timer > 0.0 {
            self.season_timer -= dt;
            if self.season_timer <= 0.0 {
                self.season = Season::None;
                self.season_timer = 0.0;
            }
        }

        // 4. Shimmer (Golden / Wrath Cookie) lifecycle
        if let Some(shimmer) = &mut self.active_shimmer {
            shimmer.time_left -= dt;
            if shimmer.time_left <= 0.0 {
                self.active_shimmer = None;
            }
        } else {
            self.shimmer_spawn_timer -= dt * self.golden_cookie_frequency_mult();
            if self.shimmer_spawn_timer <= 0.0 {
                self.spawn_shimmer();
            }
        }

        // 5. Wrinkler spawning when Grandmapocalypse is active
        let eff_stage = self.effective_grandmapocalypse_stage();
        if eff_stage > 0 && self.active_wrinkler_count() < MAX_WRINKLERS {
            self.wrinkler_spawn_timer -= dt * (eff_stage as f64);
            if self.wrinkler_spawn_timer <= 0.0 {
                if let Some(slot) = self.wrinklers.iter_mut().find(|w| !w.active) {
                    slot.active = true;
                    slot.cookies_sucked = 0.0;
                    self.last_event_message =
                        "A wrinkler has latched onto the big cookie!".to_string();
                }
                self.wrinkler_spawn_timer = 30.0;
            }
        }

        // 6. Sugar lump ripening (1 lump every 180s of active play)
        self.sugar_lump_timer += dt;
        while self.sugar_lump_timer >= 180.0 {
            self.sugar_lump_timer -= 180.0;
            self.sugar_lumps += 1;
            self.last_event_message =
                "A sweet Sugar Lump has ripened and been harvested!".to_string();
        }

        // 7. Minigames tick: Grimoire magic refill, Garden growth, Stock Market prices
        self.max_magic = 15.0 + (self.buildings[7] as f64) * 1.5;
        if self.magic < self.max_magic {
            self.magic = (self.magic + dt * 0.25).min(self.max_magic);
        }

        self.garden_tick_timer += dt;
        while self.garden_tick_timer >= 5.0 {
            self.garden_tick_timer -= 5.0;
            for row in &mut self.garden_plots {
                for cell in row.iter_mut().flatten() {
                    cell.1 = cell.1.saturating_add(1);
                }
            }
        }

        self.stock_tick_timer += dt;
        while self.stock_tick_timer >= 6.0 {
            self.stock_tick_timer -= 6.0;
            for (idx, price) in self.stock_prices.iter_mut().enumerate() {
                let baseline = 15.0 * (idx as f64 + 1.0);
                let delta = self.rng.gen_range(-4.5..4.8) + (baseline - *price) * 0.08;
                *price = (*price + delta).clamp(2.0, 500.0);
            }
        }

        // 8. News ticker rotation
        self.news_timer -= dt;
        if self.news_timer <= 0.0 {
            self.news_timer = 9.0;
            self.rotate_news();
        }

        // 9. Check achievements
        self.check_achievements();
    }

    fn golden_cookie_frequency_mult(&self) -> f64 {
        let mut mult = 1.0;
        for (i, upg) in UPGRADES.iter().enumerate() {
            if self.upgrades_purchased.get(i).copied().unwrap_or(false) {
                if let UpgradeEffect::GoldenCookieBoost { freq_factor, .. } = upg.effect {
                    mult *= freq_factor;
                }
            }
        }
        for (i, hupg) in HEAVENLY_UPGRADES.iter().enumerate() {
            if self
                .heavenly_upgrades_purchased
                .get(i)
                .copied()
                .unwrap_or(false)
            {
                mult *= hupg.golden_freq_mult;
            }
        }
        if self.season == Season::BusinessDay {
            mult *= 1.05;
        }
        mult
    }

    fn golden_cookie_duration_mult(&self) -> f64 {
        let mut mult = 1.0;
        for (i, upg) in UPGRADES.iter().enumerate() {
            if self.upgrades_purchased.get(i).copied().unwrap_or(false) {
                if let UpgradeEffect::GoldenCookieBoost {
                    effect_duration_factor,
                    ..
                } = upg.effect
                {
                    mult *= effect_duration_factor;
                }
            }
        }
        mult
    }

    pub fn spawn_shimmer(&mut self) {
        let stage = self.effective_grandmapocalypse_stage();
        let wrath_chance = match stage {
            0 => 0.0,
            1 => 0.33,
            2 => 0.66,
            _ => 1.0,
        };
        let is_wrath = self.rng.gen_bool(wrath_chance);
        let duration = 13.0 * self.golden_cookie_duration_mult().min(2.0);
        self.active_shimmer = Some(ActiveShimmer {
            is_wrath,
            time_left: duration,
            max_time: duration,
            x_norm: self.rng.gen_range(0.15..0.85),
            y_norm: self.rng.gen_range(0.15..0.85),
        });
        self.shimmer_spawn_timer = self.rng.gen_range(40.0..75.0);
        self.last_event_message = if is_wrath {
            "A glowing red Wrath Cookie has appeared! Press [G] to click it!".to_string()
        } else {
            "A shimmering Golden Cookie has appeared! Press [G] to click it!".to_string()
        };
    }

    fn rotate_news(&mut self) {
        let pool = if self.effective_grandmapocalypse_stage() > 0 && self.rng.gen_bool(0.5) {
            NEWS_GRANDMAPOCALYPSE
        } else if self.cookies_baked_this_run >= 1_000_000_000.0 {
            NEWS_LATE
        } else if self.cookies_baked_this_run >= 10_000.0 {
            NEWS_MID
        } else {
            NEWS_EARLY
        };
        let idx = self.rng.gen_range(0..pool.len());
        self.current_news = pool[idx].to_string();
    }

    /// Applies a player or UI action to the game state. Returns `true` if the action succeeded.
    pub fn apply(&mut self, action: Action) -> bool {
        match action {
            Action::ClickCookie => {
                let gain = self.cookies_per_click();
                self.bake_cookies(gain);
                self.total_clicks += 1;
                self.handmade_cookies += gain;
                self.last_click_gain = gain;
                self.check_achievements();
                true
            }
            Action::BuyOrSellBuilding(building_id) => match self.store_mode {
                StoreMode::Buy => self.apply(Action::BuyBuilding(building_id, self.buy_amount)),
                StoreMode::Sell => self.apply(Action::SellBuilding(building_id, self.buy_amount)),
            },
            Action::BuyBuilding(building_id, amount) => {
                if building_id >= NUM_BUILDINGS {
                    return false;
                }
                let (qty, cost) = self.store_quote(building_id, StoreMode::Buy, amount);
                if qty == 0 || self.cookies < cost {
                    return false;
                }
                self.cookies -= cost;
                self.buildings[building_id] = self.buildings[building_id].saturating_add(qty);
                self.last_event_message = format!(
                    "Bought {}x {} for {} cookies.",
                    qty,
                    BUILDINGS[building_id].name,
                    crate::format::format_number(cost)
                );
                self.check_achievements();
                true
            }
            Action::SellBuilding(building_id, amount) => {
                if building_id >= NUM_BUILDINGS {
                    return false;
                }
                let (qty, refund) = self.store_quote(building_id, StoreMode::Sell, amount);
                if qty == 0 {
                    return false;
                }
                self.buildings[building_id] = self.buildings[building_id].saturating_sub(qty);
                self.cookies += refund;
                self.last_event_message = format!(
                    "Sold {}x {} for {} cookies.",
                    qty,
                    BUILDINGS[building_id].name,
                    crate::format::format_number(refund)
                );
                true
            }
            Action::BuyUpgrade(upgrade_id) => {
                if upgrade_id >= UPGRADES.len() {
                    return false;
                }
                if self
                    .upgrades_purchased
                    .get(upgrade_id)
                    .copied()
                    .unwrap_or(false)
                {
                    return false;
                }
                if !self.is_upgrade_unlocked(upgrade_id) {
                    return false;
                }
                let upg = &UPGRADES[upgrade_id];
                if self.cookies < upg.cost {
                    return false;
                }
                self.cookies -= upg.cost;
                if let UpgradeEffect::GrandmapocalypseTransition {
                    target_stage,
                    pledge,
                } = upg.effect
                {
                    if pledge {
                        self.elder_pledge_timer = 300.0;
                        // Elder Pledge can be bought repeatedly so we don't mark it permanently purchased
                        self.pop_all_wrinklers_internal();
                        self.last_event_message =
                            "Elder Pledge enacted! The grandmatriarchs are appeased for 5 minutes."
                                .to_string();
                        return true;
                    } else {
                        self.grandmapocalypse_stage = self.grandmapocalypse_stage.max(target_stage);
                    }
                }
                self.upgrades_purchased[upgrade_id] = true;
                self.last_event_message = format!("Purchased upgrade: {}!", upg.name);
                self.check_achievements();
                true
            }
            Action::BuyNextAvailableUpgrade => {
                let avail = self.available_upgrades();
                if let Some(&first_id) = avail.first() {
                    self.apply(Action::BuyUpgrade(first_id))
                } else {
                    false
                }
            }
            Action::SetBuyAmount(amt) => {
                self.buy_amount = amt;
                true
            }
            Action::CycleBuyAmount => {
                self.buy_amount = self.buy_amount.next();
                true
            }
            Action::ToggleStoreMode => {
                self.store_mode = self.store_mode.toggle();
                true
            }
            Action::ClickShimmer => {
                let Some(shimmer) = self.active_shimmer.take() else {
                    return false;
                };
                self.golden_clicks += 1;
                let dur_mult = self.golden_cookie_duration_mult();
                let roll: f64 = self.rng.gen_range(0.0..1.0);

                if shimmer.is_wrath {
                    if roll < 0.35 {
                        let dur = 66.0 * dur_mult;
                        self.add_or_refresh_buff(BuffKind::Clot, dur);
                        self.last_event_message =
                            "Wrath Cookie: Clot! Cookie production halved.".to_string();
                    } else if roll < 0.65 {
                        let loss = (self.cookies * 0.05).min(self.raw_cps() * 600.0) + 13.0;
                        let actual_loss = loss.min(self.cookies);
                        self.cookies -= actual_loss;
                        self.last_event_message = format!(
                            "Wrath Cookie: Ruin! Lost {} cookies!",
                            crate::format::format_number(actual_loss)
                        );
                    } else {
                        let dur = 6.0 * dur_mult;
                        self.add_or_refresh_buff(BuffKind::ElderFrenzy, dur);
                        self.last_event_message =
                            "Wrath Cookie: ELDER FRENZY! CpS x666!".to_string();
                    }
                } else if roll < 0.45 {
                    let dur = 77.0 * dur_mult;
                    self.add_or_refresh_buff(BuffKind::Frenzy, dur);
                    self.last_event_message =
                        "Golden Cookie: Frenzy! Cookie production x7!".to_string();
                } else if roll < 0.80 {
                    let reward = (self.cookies * 0.15).max(self.raw_cps() * 900.0) + 13.0;
                    self.bake_cookies(reward);
                    self.last_event_message = format!(
                        "Golden Cookie: Lucky! +{} cookies!",
                        crate::format::format_number(reward)
                    );
                } else {
                    let dur = 13.0 * dur_mult;
                    self.add_or_refresh_buff(BuffKind::ClickFrenzy, dur);
                    self.last_event_message =
                        "Golden Cookie: CLICK FRENZY! Click power x777!".to_string();
                }

                self.check_achievements();
                true
            }
            Action::PopWrinkler(idx) => {
                if idx >= MAX_WRINKLERS || !self.wrinklers[idx].active {
                    return false;
                }
                let reward = self.wrinklers[idx].cookies_sucked * 1.1;
                self.wrinklers[idx] = Wrinkler::default();
                self.bake_cookies(reward);
                self.wrinklers_popped += 1;
                self.last_event_message = format!(
                    "Popped a wrinkler! Recovered {} cookies.",
                    crate::format::format_number(reward)
                );
                self.check_achievements();
                true
            }
            Action::PopAllWrinklers => {
                let popped = self.pop_all_wrinklers_internal();
                if popped > 0 {
                    self.check_achievements();
                    true
                } else {
                    false
                }
            }
            Action::Ascend => {
                let pending = self.pending_prestige();
                if pending == 0 && self.cookies_baked_this_run < 1_000_000.0 {
                    return false;
                }
                self.prestige_level += pending;
                self.heavenly_chips += pending;
                self.ascensions += 1;

                // Reset run state while keeping prestige, heavenly upgrades, achievements, and sugar lumps
                let mut starter_cookies = 0.0;
                for (i, hupg) in HEAVENLY_UPGRADES.iter().enumerate() {
                    if self
                        .heavenly_upgrades_purchased
                        .get(i)
                        .copied()
                        .unwrap_or(false)
                    {
                        starter_cookies += hupg.starter_cookies;
                    }
                }
                let has_starter_kit = self
                    .heavenly_upgrades_purchased
                    .get(3)
                    .copied()
                    .unwrap_or(false);

                self.cookies = starter_cookies;
                self.cookies_baked_this_run = starter_cookies;
                self.buildings = [0; NUM_BUILDINGS];
                if has_starter_kit {
                    self.buildings[0] = 10;
                }
                self.upgrades_purchased = vec![false; UPGRADES.len()];
                self.grandmapocalypse_stage = 0;
                self.elder_pledge_timer = 0.0;
                self.wrinklers = [Wrinkler::default(); MAX_WRINKLERS];
                self.active_shimmer = None;
                self.active_buffs.clear();
                self.run_time_secs = 0.0;
                self.dragon_level = 0;
                self.dragon_aura = DragonAura::None;

                self.last_event_message = format!(
                    "Ascended! Gained +{} Heavenly Chips (Prestige Level {}).",
                    pending, self.prestige_level
                );
                self.check_achievements();
                true
            }
            Action::BuyHeavenlyUpgrade(h_id) => {
                let Some(hupg) = HEAVENLY_UPGRADES.get(h_id) else {
                    return false;
                };
                if self
                    .heavenly_upgrades_purchased
                    .get(h_id)
                    .copied()
                    .unwrap_or(false)
                {
                    return false;
                }
                if self.heavenly_chips < hupg.cost_chips {
                    return false;
                }
                self.heavenly_chips -= hupg.cost_chips;
                self.heavenly_upgrades_purchased[h_id] = true;
                if hupg.starter_cookies > 0.0 {
                    self.bake_cookies(hupg.starter_cookies);
                }
                self.last_event_message = format!("Unlocked Heavenly Upgrade: {}!", hupg.name);
                true
            }
            Action::TrainDragon => {
                if self.dragon_level >= 4 {
                    return false;
                }
                let cost = 10_000.0 * 10_f64.powi(self.dragon_level as i32);
                if self.cookies < cost {
                    return false;
                }
                self.cookies -= cost;
                self.dragon_level += 1;
                let auto_aura = match self.dragon_level {
                    1 => DragonAura::BreathOfMilk,
                    2 => DragonAura::DragonCursor,
                    3 => DragonAura::ElderBattalion,
                    _ => DragonAura::RadiantAppetite,
                };
                self.dragon_aura = auto_aura;
                self.last_event_message = format!(
                    "Trained Krumblor to Stage {}! Active Aura: {}.",
                    self.dragon_level,
                    self.dragon_aura.name()
                );
                true
            }
            Action::CycleDragonAura => {
                let current_idx = DragonAura::ALL
                    .iter()
                    .position(|&a| a == self.dragon_aura)
                    .unwrap_or(0);
                for offset in 1..=DragonAura::ALL.len() {
                    let candidate = DragonAura::ALL[(current_idx + offset) % DragonAura::ALL.len()];
                    if candidate.required_dragon_level() <= self.dragon_level {
                        self.dragon_aura = candidate;
                        self.last_event_message =
                            format!("Dragon Aura set to: {}", candidate.name());
                        return true;
                    }
                }
                false
            }
            Action::SetSeason(s) => {
                let cost = (self.raw_cps() * 60.0).max(1_000.0);
                if s != Season::None && self.cookies < cost {
                    return false;
                }
                if s != Season::None {
                    self.cookies -= cost;
                    self.season_timer = 600.0;
                } else {
                    self.season_timer = 0.0;
                }
                self.season = s;
                self.last_event_message = format!("Season switched to: {}", s.name());
                true
            }
            Action::CycleSeason => {
                let idx = Season::ALL
                    .iter()
                    .position(|&s| s == self.season)
                    .unwrap_or(0);
                let next = Season::ALL[(idx + 1) % Season::ALL.len()];
                self.apply(Action::SetSeason(next))
            }
            Action::LevelUpBuilding(building_id) => {
                if building_id >= NUM_BUILDINGS {
                    return false;
                }
                let needed_lumps = self.building_levels[building_id] + 1;
                if self.sugar_lumps < needed_lumps {
                    return false;
                }
                self.sugar_lumps -= needed_lumps;
                self.building_levels[building_id] += 1;
                self.last_event_message = format!(
                    "Leveled up {} to Level {} (+{}% CpS)!",
                    BUILDINGS[building_id].name,
                    self.building_levels[building_id],
                    self.building_levels[building_id]
                );
                true
            }
            Action::GardenSelectSeed(seed_id) => {
                if seed_id >= GARDEN_SEEDS.len() {
                    return false;
                }
                self.garden_selected_seed = seed_id;
                true
            }
            Action::GardenInteractPlot(r, c) => {
                if r >= 4 || c >= 4 {
                    return false;
                }
                if let Some((seed_id, age)) = self.garden_plots[r][c] {
                    self.garden_plots[r][c] = None;
                    let seed = &GARDEN_SEEDS[seed_id];
                    if age >= seed.mature_ticks {
                        let harvest_bonus = (self.raw_cps() * 30.0).max(50.0);
                        self.bake_cookies(harvest_bonus);
                        self.last_event_message = format!(
                            "Harvested mature {} (+{} cookies)!",
                            seed.name,
                            crate::format::format_number(harvest_bonus)
                        );
                    } else {
                        self.last_event_message = format!("Uprooted {}.", seed.name);
                    }
                    true
                } else {
                    let cost = (self.raw_cps() * 15.0).max(25.0);
                    if self.cookies < cost {
                        return false;
                    }
                    self.cookies -= cost;
                    self.garden_plots[r][c] = Some((self.garden_selected_seed, 0));
                    self.last_event_message = format!(
                        "Planted {} in plot ({}, {}).",
                        GARDEN_SEEDS[self.garden_selected_seed].name,
                        r + 1,
                        c + 1
                    );
                    true
                }
            }
            Action::CastSpell(spell_id) => {
                let Some(spell) = GRIMOIRE_SPELLS.get(spell_id) else {
                    return false;
                };
                if self.magic < spell.magic_cost {
                    return false;
                }
                self.magic -= spell.magic_cost;
                match spell_id {
                    0 => {
                        let gain = (self.raw_cps() * 1_800.0)
                            .min(self.cookies * 0.15 + 100.0)
                            .max(25.0);
                        self.bake_cookies(gain);
                        self.last_event_message = format!(
                            "Cast Conjure Baked Goods! +{} cookies!",
                            crate::format::format_number(gain)
                        );
                    }
                    1 => {
                        self.spawn_shimmer();
                        self.last_event_message =
                            "Cast Force the Hand of Fate! A shimmer materialized!".to_string();
                    }
                    _ => {
                        for b in &mut self.active_buffs {
                            b.time_left *= 1.20;
                            b.max_time = b.max_time.max(b.time_left);
                        }
                        self.last_event_message =
                            "Cast Stretch Time! Active buffs extended by 20%!".to_string();
                    }
                }
                true
            }
            Action::BuyStock(idx) => {
                if idx >= STOCK_SYMBOLS.len() {
                    return false;
                }
                let unit_cookie_cost = self.stock_prices[idx] * self.raw_cps().max(1.0);
                if self.cookies < unit_cookie_cost {
                    return false;
                }
                self.cookies -= unit_cookie_cost;
                self.stock_owned[idx] += 1;
                self.last_event_message = format!(
                    "Bought 1 share of {} at ${:.2}.",
                    STOCK_SYMBOLS[idx], self.stock_prices[idx]
                );
                true
            }
            Action::SellStock(idx) => {
                if idx >= STOCK_SYMBOLS.len() || self.stock_owned[idx] == 0 {
                    return false;
                }
                let payout = self.stock_prices[idx] * self.raw_cps().max(1.0);
                self.stock_owned[idx] -= 1;
                self.bake_cookies(payout);
                self.last_event_message = format!(
                    "Sold 1 share of {} at ${:.2} (+{} cookies).",
                    STOCK_SYMBOLS[idx],
                    self.stock_prices[idx],
                    crate::format::format_number(payout)
                );
                true
            }
            Action::CyclePantheonSpirit => {
                self.pantheon_spirit = (self.pantheon_spirit + 1) % PANTHEON_SPIRITS.len();
                self.last_event_message = format!(
                    "Slotted Pantheon Spirit: {}",
                    PANTHEON_SPIRITS[self.pantheon_spirit].name
                );
                true
            }
        }
    }

    fn add_or_refresh_buff(&mut self, kind: BuffKind, duration: f64) {
        if let Some(existing) = self.active_buffs.iter_mut().find(|b| b.kind == kind) {
            existing.time_left += duration;
            existing.max_time = existing.max_time.max(existing.time_left);
        } else {
            self.active_buffs.push(ActiveBuff {
                kind,
                time_left: duration,
                max_time: duration,
            });
        }
    }

    fn pop_all_wrinklers_internal(&mut self) -> u64 {
        let mut count = 0_u64;
        let mut total_reward = 0.0;
        for w in &mut self.wrinklers {
            if w.active {
                total_reward += w.cookies_sucked * 1.1;
                *w = Wrinkler::default();
                count += 1;
            }
        }
        if count > 0 {
            self.bake_cookies(total_reward);
            self.wrinklers_popped += count;
            self.last_event_message = format!(
                "Popped {} wrinkler(s) for {} cookies!",
                count,
                crate::format::format_number(total_reward)
            );
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_cost_progression_and_bulk() {
        assert_eq!(GameState::single_building_cost(0, 0), 15.0);
        assert_eq!(GameState::single_building_cost(0, 1), 18.0); // ceil(15 * 1.15) = 18
        assert_eq!(GameState::single_building_cost(1, 0), 100.0);
        assert_eq!(GameState::single_building_cost(1, 1), 115.0);

        let mut state = GameState::with_seed(42);
        state.cookies = 1_000.0;
        assert!(state.apply(Action::BuyBuilding(0, BuyAmount::Ten)));
        assert_eq!(state.buildings[0], 10);
        assert!(state.cookies < 1_000.0);

        let cookies_before_sell = state.cookies;
        assert!(state.apply(Action::SellBuilding(0, BuyAmount::One)));
        assert_eq!(state.buildings[0], 9);
        assert!(state.cookies > cookies_before_sell);
    }

    #[test]
    fn test_buy_max() {
        let mut state = GameState::with_seed(42);
        state.cookies = 50.0; // Can afford 2 cursors: 15 + 18 = 33 (3rd costs 21, 33+21=54 > 50)
        assert!(state.apply(Action::BuyBuilding(0, BuyAmount::Max)));
        assert_eq!(state.buildings[0], 2);
        assert_eq!(state.cookies, 17.0);
    }

    #[test]
    fn test_upgrades_and_cps_calculation() {
        let mut state = GameState::with_seed(42);
        state.bake_cookies(10_000.0);
        state.apply(Action::BuyBuilding(0, BuyAmount::One));
        assert!((state.raw_cps() - 0.1).abs() < 1e-6);
        assert!((state.cookies_per_click() - 1.0).abs() < 1e-6);

        // Buy "Reinforced index finger" (id 0, cost 100)
        assert!(state.apply(Action::BuyUpgrade(0)));
        assert!((state.raw_cps() - 0.2).abs() < 1e-6);
        assert!((state.cookies_per_click() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_golden_cookie_and_buffs() {
        let mut state = GameState::with_seed(7);
        state.bake_cookies(1_000.0);
        state.apply(Action::BuyBuilding(1, BuyAmount::One)); // 1 Grandma = 1.0 CPS
        state.spawn_shimmer();
        assert!(state.active_shimmer.is_some());
        assert!(state.apply(Action::ClickShimmer));
        assert_eq!(state.golden_clicks, 1);
    }

    #[test]
    fn test_wrinklers_leech_and_pop() {
        let mut state = GameState::with_seed(99);
        state.buildings[1] = 10; // 10 CPS
        state.wrinklers[0].active = true;

        assert!((state.raw_cps() - 10.0).abs() < 1e-6);
        assert!((state.effective_cps() - 9.5).abs() < 1e-6);

        state.tick(10.0);
        assert!((state.wrinklers[0].cookies_sucked - 5.0).abs() < 1e-6);

        let before = state.cookies;
        assert!(state.apply(Action::PopWrinkler(0)));
        assert!(!state.wrinklers[0].active);
        assert!((state.cookies - (before + 5.5)).abs() < 1e-6);
        assert_eq!(state.wrinklers_popped, 1);
    }

    #[test]
    fn test_ascension_and_heavenly_chips() {
        let mut state = GameState::with_seed(123);
        state.bake_cookies(8_000_000_000_000.0); // 8e12 -> cbrt(8) = 2 prestige levels
        assert_eq!(state.pending_prestige(), 2);

        assert!(state.apply(Action::Ascend));
        assert_eq!(state.prestige_level, 2);
        assert_eq!(state.heavenly_chips, 2);
        assert_eq!(state.ascensions, 1);
        assert_eq!(state.cookies, 0.0);

        // Buy Legacy (id 0, cost 1 chip)
        assert!(state.apply(Action::BuyHeavenlyUpgrade(0)));
        assert_eq!(state.heavenly_chips, 1);
        assert!(state.heavenly_upgrades_purchased[0]);
    }

    #[test]
    fn test_minigames_and_dragon() {
        let mut state = GameState::with_seed(55);
        state.bake_cookies(10_000_000.0);
        state.buildings[7] = 5; // 5 Wizard Towers

        // Train dragon
        assert!(state.apply(Action::TrainDragon));
        assert_eq!(state.dragon_level, 1);
        assert_eq!(state.dragon_aura, DragonAura::BreathOfMilk);

        // Garden plant and mature
        assert!(state.apply(Action::GardenInteractPlot(0, 0)));
        assert!(state.garden_plots[0][0].is_some());
        state.tick(60.0); // Advances garden ticks
        assert!(state.apply(Action::GardenInteractPlot(0, 0)));
        assert!(state.garden_plots[0][0].is_none());

        // Grimoire spell
        state.magic = 30.0;
        assert!(state.apply(Action::CastSpell(0)));
    }
}
