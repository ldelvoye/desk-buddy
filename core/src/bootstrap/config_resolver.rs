//! Bootstrap configuration resolver with precedence: env -> config file -> defaults.
use crate::bootstrap::core_builder::CoreOptions;
use crate::error::{CoreError, CoreResult};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Resolved core options passed into the bootstrap builder.
    pub options: CoreOptions,
}

#[derive(Debug, Default, Deserialize)]
struct FileConfig {
    bootstrap: Option<BootstrapFileConfig>,
}

#[derive(Debug, Default, Deserialize)]
struct BootstrapFileConfig {
    database_url: Option<String>,
}

pub struct ConfigResolver;

impl ConfigResolver {
    /// Loads bootstrap config using precedence: env -> `config.toml` -> defaults.
    pub fn load() -> CoreResult<RuntimeConfig> {
        Self::load_from_path(Path::new("config.toml"))
    }

    fn load_from_path(path: &Path) -> CoreResult<RuntimeConfig> {
        let file_config: FileConfig = read_file_config(path)?;
        let database_url: String = resolve_database_url(&file_config)?;
        ensure_database_parent_directory(&database_url)?;

        Ok(RuntimeConfig {
            options: CoreOptions { database_url },
        })
    }
}

/// Reads optional TOML config from disk, defaulting to empty config when absent.
fn read_file_config(path: &Path) -> CoreResult<FileConfig> {
    if !path.exists() {
        return Ok(FileConfig::default());
    }

    let raw: String = fs::read_to_string(path)?;
    if raw.trim().is_empty() {
        return Ok(FileConfig::default());
    }

    let parsed: FileConfig = toml::from_str(&raw)?;
    Ok(parsed)
}

/// Resolves database URL precedence: env -> config file -> default path.
fn resolve_database_url(file_config: &FileConfig) -> CoreResult<String> {
    match env::var("DESK_BUDDY_DATABASE_URL") {
        Ok(value) => validate_non_empty(
            "DESK_BUDDY_DATABASE_URL",
            value,
            "DESK_BUDDY_DATABASE_URL",
        ),
        Err(env::VarError::NotPresent) => match &file_config.bootstrap {
            Some(bootstrap) => match &bootstrap.database_url {
                Some(value) => validate_non_empty("bootstrap.database_url", value.clone(), "sqlite URL"),
                None => Ok(CoreOptions::default().database_url),
            },
            None => Ok(CoreOptions::default().database_url),
        },
        Err(env::VarError::NotUnicode(_)) => Err(CoreError::InvalidConfigValue {
            field: "DESK_BUDDY_DATABASE_URL",
            value: "<non-unicode>".to_string(),
            expected: "utf-8 sqlite URL",
        }),
    }
}

/// Rejects empty string values for required config fields.
fn validate_non_empty(
    field: &'static str,
    value: String,
    expected: &'static str,
) -> CoreResult<String> {
    let trimmed: String = value.trim().to_string();
    if trimmed.is_empty() {
        return Err(CoreError::InvalidConfigValue {
            field,
            value,
            expected,
        });
    }

    Ok(trimmed)
}

/// Creates the parent directory for file-based SQLite URLs when needed.
fn ensure_database_parent_directory(database_url: &str) -> CoreResult<()> {
    if database_url == "sqlite::memory:" {
        return Ok(());
    }

    let Some(path_portion) = database_url.strip_prefix("sqlite://") else {
        return Ok(());
    };

    let db_path: PathBuf = PathBuf::from(path_portion);
    if let Some(parent) = db_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(())
}
