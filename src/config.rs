use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_speed")]
    pub speed: u64,
    #[serde(default = "default_background")]
    pub background: bool,
    #[serde(default = "default_order")]
    pub order: String,
    #[serde(default = "default_loop")]
    pub loop_playback: bool,
}

fn default_theme() -> String {
    "tokyo-night".to_string()
}

fn default_speed() -> u64 {
    30
}

fn default_background() -> bool {
    true
}

fn default_order() -> String {
    "random".to_string()
}

fn default_loop() -> bool {
    false
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            speed: default_speed(),
            background: default_background(),
            order: default_order(),
            loop_playback: default_loop(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        let contents = if config_path.exists() {
            // Load existing config and update values to preserve comments
            let existing = fs::read_to_string(&config_path).with_context(|| {
                format!("Failed to read config file: {}", config_path.display())
            })?;

            let mut doc = existing
                .parse::<toml_edit::DocumentMut>()
                .with_context(|| {
                    format!("Failed to parse config file: {}", config_path.display())
                })?;

            // Update values while preserving comments
            doc["theme"] = toml_edit::value(self.theme.as_str());
            doc["speed"] = toml_edit::value(self.speed as i64);
            doc["background"] = toml_edit::value(self.background);
            doc["order"] = toml_edit::value(self.order.as_str());
            doc["loop"] = toml_edit::value(self.loop_playback);

            doc.to_string()
        } else {
            // Create new config with comments
            format!(
                "# gitlogue configuration file\n\
                 # All settings are optional and will use defaults if not specified\n\
                 \n\
                 # Theme to use for syntax highlighting\n\
                 theme = \"{}\"\n\
                 \n\
                 # Typing speed in milliseconds per character\n\
                 speed = {}\n\
                 \n\
                 # Show background colors (set to false for transparent background)\n\
                 background = {}\n\
                 \n\
                 # Commit playback order: random, asc, or desc\n\
                 order = \"{}\"\n\
                 \n\
                 # Loop the animation continuously\n\
                 loop = {}\n",
                self.theme, self.speed, self.background, self.order, self.loop_playback
            )
        };

        fs::write(&config_path, contents)
            .with_context(|| format!("Failed to write config file: {}", config_path.display()))
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::home_dir()
            .context("Failed to determine home directory")?
            .join(".config")
            .join("gitlogue");

        fs::create_dir_all(&config_dir).with_context(|| {
            format!(
                "Failed to create config directory: {}",
                config_dir.display()
            )
        })?;

        Ok(config_dir.join("config.toml"))
    }

    #[allow(dead_code)]
    pub fn themes_dir() -> Result<PathBuf> {
        let config_dir = dirs::home_dir()
            .context("Failed to determine home directory")?
            .join(".config")
            .join("gitlogue")
            .join("themes");

        fs::create_dir_all(&config_dir).with_context(|| {
            format!(
                "Failed to create themes directory: {}",
                config_dir.display()
            )
        })?;

        Ok(config_dir)
    }
}
