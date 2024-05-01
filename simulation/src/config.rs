use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }
}
