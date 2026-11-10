# Cookie-CLI

A full-featured remake of **Cookie Clicker** for the terminal, built in Rust with [`ratatui`](https://ratatui.rs/) and [`crossterm`](https://github.com/crossterm-rs/crossterm).

## Features

- **20 Canonical Buildings**: From *Cursor* and *Grandma* all the way to *Cortex Baker* and *You*, with exact `1.15^n` cost scaling and bulk buy/sell (`x1`, `x10`, `x100`, `Max`).
- **Upgrades & Synergies**: Tiered building multipliers, *Thousand/Million/Billion Fingers*, mouse CpS-to-click upgrades, Grandma synergies, building-to-building synergies, flavored cookies, and Milk-scaled Kitten upgrades.
- **Achievements & Animated Milk**: Production, CpS, Clicking, Building, Golden Cookie, Wrinkler, and Prestige achievements that raise your Milk level and unlock new Milk flavors.
- **Golden & Wrath Cookies**: Shimmer spawns with *Frenzy* (`x7 CpS`), *Click Frenzy* (`x777 Click`), *Lucky!*, *Elder Frenzy* (`x666 CpS`), *Clot*, and *Ruin!*.
- **Grandmapocalypse & Wrinklers**: Research upgrades (*One Mind*, *Communal Brainsweep*, *Elder Pact*, *Elder Pledge*) and up to 10 Wrinklers that leech CpS and burst for `110%` of swallowed cookies.
- **Ascension & Heavenly Chips**: Prestige calculation (`cbrt(cookies_all_time / 1e12)`) and permanent Heavenly Upgrades (*Legacy*, *Starter Kit*, *Heavenly Luck*, *Season Switcher*, etc.).
- **Sugar Lumps, Dragon, Seasons & Minigames**:
  - **Sugar Lumps** to permanently level up buildings (`+1% CpS` per level).
  - **Krumblor the Cookie Dragon** training and selectable Dragon Auras (*Breath of Milk*, *Dragon Cursor*, *Elder Battalion*, *Radiant Appetite*).
  - **Season Switcher** (*Christmas*, *Valentine's Day*, *Halloween*, *Easter*, *Business Day*).
  - **Garden** (4x4 plot grid with growing seeds), **Grimoire** (magic meter & spells), **Stock Market** (6 tradable cookie commodities), and **Pantheon** (slottable spirits).
- **Smooth 60 FPS TUI & Persistence**: Pulse/squash Big Cookie animations, falling crumb particles, floating `+N` popups, scrolling news ticker, atomic JSON autosave with `.bak` backup, offline CpS progress on load, and Base64 save string export/import.

## CLI Options

```bash
cookie-cli [OPTIONS]

Options:
      --save-path <SAVE_PATH>  Custom path for the JSON save file
      --fps <FPS>              Target rendering frames per second (10..=120) [default: 60]
      --no-color               Disable ANSI colors for monochrome terminals
      --ascii                  Use pure 7-bit ASCII icons and borders instead of Unicode/Emoji
      --reset                  Start a fresh game, overwriting any existing save on exit
  -h, --help                   Print help
  -V, --version                Print version
```

## Keyboard Shortcuts

| Key | Action |
| --- | --- |
| `Space` / `Enter` | Click the Big Cookie |
| `Up` / `Down` | Navigate Buildings, Upgrades, Heavenly Upgrades, or Stocks |
| `Right` / `Y` | Buy or Sell the selected Building |
| `B` | Toggle Store Mode (`BUY` / `SELL`) |
| `X` | Cycle Bulk Amount (`x1` / `x10` / `x100` / `Max`) |
| `U` | Buy selected or cheapest available Upgrade |
| `G` | Click active Golden / Wrath Cookie shimmer |
| `W` | Pop all active Wrinklers (`110%` cookie return) |
| `L` | Spend Sugar Lump to level up selected Building |
| `D` / `A` | Train Krumblor Dragon / Cycle Dragon Aura |
| `V` | Cycle festive Season |
| `Z` / `C` / `F` | Cast Grimoire Spell 1 / 2 / 3 |
| `P` / `M` | Plant or Harvest Garden Plot / Cycle Seed |
| `K` / `J` / `N` | Buy Stock / Sell Stock / Cycle Pantheon Spirit |
| `R` / `H` | Ascend (Prestige) / Buy selected Heavenly Upgrade |
| `S` / `E` / `I` | Save Game / Export Base64 / Import Base64 |
| `Tab` / `1`-`6` | Switch Center Panel Tab |
| `Q` / `Esc` | Save and Quit |

## Build & Test

```bash
# Format check
cargo fmt --check

# Lint check
cargo clippy --all-targets -- -D warnings

# Run unit & headless TUI rendering tests
cargo test

# Build optimized release binary
cargo build --release
```
