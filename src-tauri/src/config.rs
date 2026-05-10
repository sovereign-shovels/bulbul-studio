use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub endpoint: Option<String>,
    pub api_key_env_var: Option<String>,
    pub language: Option<String>,
    pub speaker: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct TomlConfig {
    provider: Option<Config>,
}

impl Config {
    pub fn load() -> Self {
        let mut merged = Self::default();

        if let Some(path) = Self::path() {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(toml) = toml::from_str::<TomlConfig>(&content) {
                    if let Some(p) = toml.provider {
                        merged.endpoint = p.endpoint.or(merged.endpoint);
                        merged.api_key_env_var = p.api_key_env_var.or(merged.api_key_env_var);
                        merged.language = p.language.or(merged.language);
                        merged.speaker = p.speaker.or(merged.speaker);
                        merged.model = p.model.or(merged.model);
                    }
                }
            }
        }

        if let Ok(v) = env::var("BULBUL_STUDIO_ENDPOINT") {
            merged.endpoint = Some(v);
        }
        if let Ok(v) = env::var("BULBUL_STUDIO_API_KEY_ENV") {
            merged.api_key_env_var = Some(v);
        }
        if let Ok(v) = env::var("BULBUL_STUDIO_LANGUAGE") {
            merged.language = Some(v);
        }
        if let Ok(v) = env::var("BULBUL_STUDIO_SPEAKER") {
            merged.speaker = Some(v);
        }
        if let Ok(v) = env::var("BULBUL_STUDIO_MODEL") {
            merged.model = Some(v);
        }

        if merged.endpoint.is_none() {
            merged.endpoint = Some("https://api.sarvam.ai/text-to-speech".into());
        }
        if merged.language.is_none() {
            merged.language = Some("hi-IN".into());
        }
        if merged.speaker.is_none() {
            merged.speaker = Some("meera".into());
        }
        if merged.model.is_none() {
            merged.model = Some("bulbul:v3".into());
        }

        merged
    }

    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("bulbul-studio").join("config.toml"))
    }

    pub fn api_key(&self) -> Option<String> {
        self.api_key_env_var.as_ref().and_then(|k| env::var(k).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let cfg = Config::load();
        assert!(cfg.endpoint.is_some());
        assert!(cfg.language.is_some());
        assert!(cfg.speaker.is_some());
        assert!(cfg.model.is_some());
        assert_eq!(cfg.language.unwrap(), "hi-IN");
        assert_eq!(cfg.speaker.unwrap(), "meera");
        assert_eq!(cfg.model.unwrap(), "bulbul:v3");
    }
}
