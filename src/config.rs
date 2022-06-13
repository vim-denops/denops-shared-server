use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use shellexpand::full_with_context_no_errors;
use std::env;
use std::path::{Path, PathBuf};

const DENOPS_SHARED_SERVER_CONFIG: &str = "DENOPS_SHARED_SERVER_CONFIG";
const DEFAULT_DENOPS_SHARED_SERVER_CONFIG: &str = "~/.denops_shared_server/config.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    root: PathBuf,
    host: String,
    port: u64,
}

impl Config {
    pub fn path() -> PathBuf {
        let f = env::var(DENOPS_SHARED_SERVER_CONFIG)
            .unwrap_or(DEFAULT_DENOPS_SHARED_SERVER_CONFIG.into());
        let f = full_with_context_no_errors(&f, env::home_dir, |s| -> Option<String> {
            match env::var(s) {
                Ok(value) => Some(value.into()),
                Err(_) => None,
            }
        });
        Path::new(f.as_ref()).to_path_buf()
    }

    pub fn load<P: AsRef<Path>>(p: P) -> Result<Self> {
        let t = std::fs::read_to_string(p)?;
        let c = from_str(&t)?;
        Ok(c)
    }

    pub fn save<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        let t = to_string(self)?;
        std::fs::write(p, t)?;
        Ok(())
    }
}
