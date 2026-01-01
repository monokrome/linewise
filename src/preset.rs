//! Linewise preset system
//!
//! Presets define how to parse, transform, and display record-oriented data.
//!
//! # Preset File Format (TOML)
//!
//! ```toml
//! [preset]
//! name = "bl4-items"
//! description = "Borderlands 4 item serials"
//!
//! # How to detect records (default: newline-delimited)
//! [records]
//! format = "lines"  # or "length16", "length32", "custom"
//! # For custom: pattern = "..."  # regex for record boundaries
//!
//! # Detection rules - how to identify this preset automatically
//! [[detect]]
//! type = "starts_with"
//! value = "@Ug"
//!
//! [[detect]]
//! type = "min_length"
//! value = 20
//!
//! # Gloss transforms - show decoded/translated values
//! [gloss]
//! # Built-in transforms: base85, base64, hex, none
//! transform = "base85"
//! # Or use an external command:
//! # command = ["bl4", "serial", "decode", "--json"]
//!
//! # Coloring rules for display
//! [[color]]
//! match = "^@Ug"
//! style = "green bold"
//!
//! [[color]]
//! match = "Legendary"
//! style = "yellow"
//!
//! # Custom field extraction for structured display
//! [[fields]]
//! name = "serial"
//! pattern = "^(@[A-Za-z0-9+/=~!@#$%^&*]+)"
//!
//! [[fields]]
//! name = "rarity"
//! from_gloss = true
//! pattern = "Rarity: (\\w+)"
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;

/// A complete preset definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    #[serde(default)]
    pub preset: PresetMeta,
    #[serde(default)]
    pub records: RecordFormat,
    #[serde(default)]
    pub detect: Vec<DetectRule>,
    #[serde(default)]
    pub gloss: Option<GlossConfig>,
    #[serde(default)]
    pub color: Vec<ColorRule>,
    #[serde(default)]
    pub fields: Vec<FieldExtractor>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PresetMeta {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// How records are delimited in the input
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format", rename_all = "lowercase")]
pub enum RecordFormat {
    /// Newline-delimited text lines
    Lines,
    /// Binary with u16 length prefix
    Length16,
    /// Binary with u32 length prefix
    Length32,
    /// Custom regex pattern for boundaries
    Custom { pattern: String },
}

impl Default for RecordFormat {
    fn default() -> Self {
        Self::Lines
    }
}

/// Rule for auto-detecting which preset to use
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DetectRule {
    StartsWith { value: String },
    EndsWith { value: String },
    Contains { value: String },
    Regex { pattern: String },
    MinLength { value: usize },
    MaxLength { value: usize },
    ByteEquals { position: usize, value: u8 },
}

impl DetectRule {
    /// Check if a record matches this rule
    pub fn matches(&self, record: &[u8]) -> bool {
        match self {
            Self::StartsWith { value } => {
                let s = String::from_utf8_lossy(record);
                s.starts_with(value)
            }
            Self::EndsWith { value } => {
                let s = String::from_utf8_lossy(record);
                s.ends_with(value)
            }
            Self::Contains { value } => {
                let s = String::from_utf8_lossy(record);
                s.contains(value)
            }
            Self::Regex { pattern } => {
                // TODO: compile regex once
                let s = String::from_utf8_lossy(record);
                regex::Regex::new(pattern)
                    .map(|re| re.is_match(&s))
                    .unwrap_or(false)
            }
            Self::MinLength { value } => record.len() >= *value,
            Self::MaxLength { value } => record.len() <= *value,
            Self::ByteEquals { position, value } => record.get(*position) == Some(value),
        }
    }
}

/// Configuration for gloss (decode/transform) display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossConfig {
    /// Built-in transform: base85, base64, hex, none
    #[serde(default)]
    pub transform: Option<String>,
    /// External command to run for transformation
    #[serde(default)]
    pub command: Option<Vec<String>>,
    /// Cache transformed results
    #[serde(default = "default_true")]
    pub cache: bool,
}

fn default_true() -> bool {
    true
}

impl GlossConfig {
    /// Apply the gloss transform to a record
    pub async fn apply(&self, record: &str) -> Result<String> {
        // Try built-in transform first
        if let Some(transform) = &self.transform {
            return self.apply_builtin(transform, record);
        }

        // Try external command
        if let Some(cmd) = &self.command {
            return self.apply_command(cmd, record).await;
        }

        Ok(record.to_string())
    }

    fn apply_builtin(&self, transform: &str, record: &str) -> Result<String> {
        match transform {
            "base85" => {
                // Decode base85 and show as hex
                let bytes = base85::decode(record)
                    .map_err(|e| anyhow::anyhow!("base85 decode error: {:?}", e))?;
                Ok(hex::encode(&bytes))
            }
            "base64" => {
                use base64::Engine;
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(record)
                    .context("base64 decode error")?;
                Ok(hex::encode(&bytes))
            }
            "hex" => {
                // Already hex, just clean it up
                Ok(record.replace([' ', '\n', '\r'], ""))
            }
            "none" | "" => Ok(record.to_string()),
            _ => Err(anyhow::anyhow!("unknown transform: {}", transform)),
        }
    }

    async fn apply_command(&self, cmd: &[String], record: &str) -> Result<String> {
        if cmd.is_empty() {
            return Ok(record.to_string());
        }

        let output = Command::new(&cmd[0])
            .args(&cmd[1..])
            .arg(record)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .context("failed to run gloss command")?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("gloss command failed: {}", stderr))
        }
    }
}

/// Rule for coloring output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorRule {
    /// Regex pattern to match
    #[serde(rename = "match")]
    pub pattern: String,
    /// Style specification: "red", "green bold", "yellow underline", etc.
    pub style: String,
}

/// Extract structured fields from records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldExtractor {
    /// Field name
    pub name: String,
    /// Regex pattern with capture group
    pub pattern: String,
    /// Extract from glossed output instead of raw
    #[serde(default)]
    pub from_gloss: bool,
}

/// Preset manager - loads and caches presets
#[derive(Debug, Default)]
pub struct PresetManager {
    presets: HashMap<String, Preset>,
    search_paths: Vec<PathBuf>,
}

impl PresetManager {
    pub fn new() -> Self {
        let mut mgr = Self::default();

        // Add default search paths
        if let Ok(home) = std::env::var("HOME") {
            mgr.search_paths
                .push(PathBuf::from(format!("{}/.config/linewise/presets", home)));
        }

        // XDG config
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            mgr.search_paths
                .push(PathBuf::from(format!("{}/linewise/presets", xdg)));
        }

        // System paths
        mgr.search_paths
            .push(PathBuf::from("/etc/linewise/presets"));
        mgr.search_paths
            .push(PathBuf::from("/usr/share/linewise/presets"));

        mgr
    }

    /// Add a custom search path
    pub fn add_search_path(&mut self, path: impl Into<PathBuf>) {
        self.search_paths.insert(0, path.into());
    }

    /// Load all presets from search paths
    pub fn load_all(&mut self) -> Result<()> {
        for path in &self.search_paths.clone() {
            if path.is_dir() {
                self.load_from_dir(path)?;
            }
        }
        Ok(())
    }

    /// Load presets from a directory
    pub fn load_from_dir(&mut self, dir: &Path) -> Result<()> {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return Ok(()), // Directory doesn't exist, that's fine
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "toml").unwrap_or(false) {
                if let Err(e) = self.load_preset(&path) {
                    eprintln!("Warning: failed to load preset {:?}: {}", path, e);
                }
            }
        }

        Ok(())
    }

    /// Load a single preset file
    pub fn load_preset(&mut self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path).context("failed to read preset file")?;
        let preset: Preset = toml::from_str(&content).context("failed to parse preset")?;
        let name = preset.preset.name.clone();
        if name.is_empty() {
            let name = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            self.presets.insert(name, preset);
        } else {
            self.presets.insert(name, preset);
        }
        Ok(())
    }

    /// Get a preset by name
    pub fn get(&self, name: &str) -> Option<&Preset> {
        self.presets.get(name)
    }

    /// Auto-detect which preset to use based on sample records
    pub fn detect(&self, records: &[Vec<u8>], sample_size: usize) -> Option<&Preset> {
        use rand::seq::SliceRandom;

        if records.is_empty() || self.presets.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let samples: Vec<&Vec<u8>> = if records.len() <= sample_size {
            records.iter().collect()
        } else {
            records.choose_multiple(&mut rng, sample_size).collect()
        };

        let mut best_match: Option<(&str, usize)> = None;

        for (name, preset) in &self.presets {
            if preset.detect.is_empty() {
                continue;
            }

            let matches = samples
                .iter()
                .filter(|record| preset.detect.iter().all(|rule| rule.matches(record)))
                .count();

            let threshold = (samples.len() * 80) / 100;
            if matches >= threshold {
                match &best_match {
                    None => best_match = Some((name, matches)),
                    Some((_, best_count)) if matches > *best_count => {
                        best_match = Some((name, matches));
                    }
                    _ => {}
                }
            }
        }

        best_match.and_then(|(name, _)| self.presets.get(name))
    }

    /// List all loaded presets
    pub fn list(&self) -> Vec<&str> {
        self.presets.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_preset() {
        let toml = r#"
[preset]
name = "test"
description = "Test preset"

[records]
format = "lines"

[[detect]]
type = "starts_with"
value = "@Ug"

[[detect]]
type = "min_length"
value = 20

[gloss]
transform = "base85"

[[color]]
match = "^@"
style = "green"
"#;

        let preset: Preset = toml::from_str(toml).unwrap();
        assert_eq!(preset.preset.name, "test");
        assert_eq!(preset.detect.len(), 2);
        assert!(preset.gloss.is_some());
        assert_eq!(preset.color.len(), 1);
    }

    #[test]
    fn test_detect_rules() {
        let rule = DetectRule::StartsWith {
            value: "@Ug".to_string(),
        };
        assert!(rule.matches(b"@UgABC123"));
        assert!(!rule.matches(b"ABC@Ug"));

        let rule = DetectRule::MinLength { value: 5 };
        assert!(rule.matches(b"12345"));
        assert!(!rule.matches(b"1234"));
    }
}
