use crate::engine::GameState;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const SAVE_VERSION: u32 = 1;
const EXPORT_PREFIX: &str = "CookieCLI|1|";
const EXPORT_SUFFIX: &str = "|END";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveEnvelope {
    pub version: u32,
    pub timestamp_unix: u64,
    pub state: GameState,
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Resolves the default platform-appropriate save file path.
pub fn default_save_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "cookie-cli", "cookie-cli") {
        proj_dirs.data_dir().join("save.json")
    } else {
        PathBuf::from("cookie_save.json")
    }
}

/// Atomically saves `GameState` to `path`, maintaining a `.bak` backup of the previous save.
pub fn save_to_path(state: &GameState, path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create save dir: {e}"))?;
        }
    }

    let envelope = SaveEnvelope {
        version: SAVE_VERSION,
        timestamp_unix: current_unix_timestamp(),
        state: state.clone(),
    };

    let json =
        serde_json::to_string_pretty(&envelope).map_err(|e| format!("Serialization error: {e}"))?;

    let tmp_path = path.with_extension("json.tmp");
    let bak_path = path.with_extension("json.bak");

    fs::write(&tmp_path, json.as_bytes())
        .map_err(|e| format!("Failed to write temporary save file: {e}"))?;

    if path.exists() {
        let _ = fs::copy(path, &bak_path);
    }

    if let Err(rename_err) = fs::rename(&tmp_path, path) {
        // Fallback on Windows if rename over existing file fails
        fs::copy(&tmp_path, path)
            .map_err(|e| format!("Failed to finalize save file ({rename_err}): {e}"))?;
        let _ = fs::remove_file(&tmp_path);
    }

    Ok(())
}

/// Loads `GameState` from `path` (falling back to `.bak` if needed) and applies offline CPS progress.
/// Returns `(loaded_state, offline_cookies_earned)`.
pub fn load_from_path(path: &Path) -> Result<(GameState, f64), String> {
    let bak_path = path.with_extension("json.bak");
    let raw = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(primary_err) => {
            if bak_path.exists() {
                fs::read_to_string(&bak_path)
                    .map_err(|e| format!("Failed to read save and backup: {e}"))?
            } else {
                return Err(format!("Could not read save file: {primary_err}"));
            }
        }
    };

    let envelope: SaveEnvelope = match serde_json::from_str(&raw) {
        Ok(env) => env,
        Err(_) => {
            let state: GameState =
                serde_json::from_str(&raw).map_err(|e| format!("Invalid save file format: {e}"))?;
            SaveEnvelope {
                version: SAVE_VERSION,
                timestamp_unix: current_unix_timestamp(),
                state,
            }
        }
    };

    let mut state = envelope.state;
    state.normalize_after_load();

    let now = current_unix_timestamp();
    let elapsed_secs = now.saturating_sub(envelope.timestamp_unix).min(604_800);
    let offline_cookies = if elapsed_secs > 5 {
        let gain = state.effective_cps() * (elapsed_secs as f64) * 0.5;
        if gain > 0.0 {
            state.bake_cookies(gain);
            state.last_event_message = format!(
                "Welcome back! Earned {} cookies while offline ({}).",
                crate::format::format_number(gain),
                crate::format::format_duration(elapsed_secs as f64)
            );
        }
        gain
    } else {
        0.0
    };

    Ok((state, offline_cookies))
}

/// Exports the current `GameState` into a portable Base64 string.
pub fn export_base64(state: &GameState) -> Result<String, String> {
    let envelope = SaveEnvelope {
        version: SAVE_VERSION,
        timestamp_unix: current_unix_timestamp(),
        state: state.clone(),
    };
    let json =
        serde_json::to_string(&envelope).map_err(|e| format!("Export serialize error: {e}"))?;
    let encoded = BASE64.encode(json.as_bytes());
    Ok(format!("{EXPORT_PREFIX}{encoded}{EXPORT_SUFFIX}"))
}

/// Imports a `GameState` from a portable Base64 save string.
pub fn import_base64(input: &str) -> Result<GameState, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Empty save string".to_string());
    }
    let without_prefix = trimmed.strip_prefix(EXPORT_PREFIX).unwrap_or(trimmed);
    let clean = without_prefix
        .strip_suffix(EXPORT_SUFFIX)
        .unwrap_or(without_prefix)
        .trim();

    let bytes = BASE64
        .decode(clean.as_bytes())
        .map_err(|e| format!("Invalid Base64 save string: {e}"))?;
    let json_str =
        String::from_utf8(bytes).map_err(|e| format!("Save string is not valid UTF-8: {e}"))?;

    let mut state = if let Ok(env) = serde_json::from_str::<SaveEnvelope>(&json_str) {
        env.state
    } else {
        serde_json::from_str::<GameState>(&json_str)
            .map_err(|e| format!("Corrupted save data in string: {e}"))?
    };

    state.normalize_after_load();
    state.last_event_message = "Save imported successfully!".to_string();
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_base64_export_import_roundtrip() {
        let mut state = GameState::with_seed(42);
        state.bake_cookies(123_456.0);
        state.buildings[0] = 12;
        state.buildings[1] = 5;
        state.prestige_level = 3;

        let code = export_base64(&state).expect("export should succeed");
        assert!(code.starts_with(EXPORT_PREFIX));
        assert!(code.ends_with(EXPORT_SUFFIX));

        let imported = import_base64(&code).expect("import should succeed");
        assert!((imported.cookies - 123_456.0).abs() < 1e-6);
        assert_eq!(imported.buildings[0], 12);
        assert_eq!(imported.buildings[1], 5);
        assert_eq!(imported.prestige_level, 3);
    }

    #[test]
    fn test_atomic_save_and_load_roundtrip() {
        let temp_dir = env::temp_dir().join("cookie_cli_test_save");
        let _ = fs::remove_dir_all(&temp_dir);
        let save_file = temp_dir.join("test_save.json");

        let mut state = GameState::with_seed(99);
        state.bake_cookies(50_000.0);
        state.buildings[2] = 7;

        save_to_path(&state, &save_file).expect("first save should succeed");
        assert!(save_file.exists());

        state.bake_cookies(25_000.0);
        save_to_path(&state, &save_file).expect("second save should create .bak");
        assert!(save_file.with_extension("json.bak").exists());

        let (loaded, _offline) = load_from_path(&save_file).expect("load should succeed");
        assert!((loaded.cookies - 75_000.0).abs() < 1e-6);
        assert_eq!(loaded.buildings[2], 7);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
