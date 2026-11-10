pub mod app;
pub mod data;
pub mod engine;
pub mod format;
pub mod save;
pub mod ui;

use crate::app::{App, AppConfig};
use crate::engine::GameState;
use crate::save::{default_save_path, load_from_path, save_to_path};
use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::panic;
use std::path::PathBuf;

/// Cookie-CLI: Full-featured Cookie Clicker in your terminal.
#[derive(Debug, Parser)]
#[command(name = "cookie-cli", version, about)]
pub struct CliArgs {
    /// Custom path for the JSON save file.
    #[arg(long)]
    pub save_path: Option<PathBuf>,

    /// Target rendering frames per second (10..=120).
    #[arg(long, default_value_t = 60)]
    pub fps: u32,

    /// Disable ANSI colors for monochrome terminals.
    #[arg(long)]
    pub no_color: bool,

    /// Use pure 7-bit ASCII icons and borders instead of Unicode/Emoji.
    #[arg(long)]
    pub ascii: bool,

    /// Start a fresh game, overwriting any existing save on exit.
    #[arg(long)]
    pub reset: bool,
}

fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
}

fn install_panic_hook() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        restore_terminal();
        default_hook(panic_info);
    }));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();
    let save_path = args.save_path.unwrap_or_else(default_save_path);

    let state = if args.reset || !save_path.exists() {
        GameState::new()
    } else {
        match load_from_path(&save_path) {
            Ok((loaded, _offline)) => loaded,
            Err(_) => GameState::new(),
        }
    };

    install_panic_hook();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let config = AppConfig {
        save_path: save_path.clone(),
        fps: args.fps,
        ascii_only: args.ascii,
        no_color: args.no_color,
    };

    let mut app = App::new(state, config);
    let run_res = app.run(&mut terminal);

    restore_terminal();
    let _ = save_to_path(&app.state, &save_path);

    if let Err(err) = run_res {
        eprintln!("Error: {err}");
    }

    Ok(())
}
