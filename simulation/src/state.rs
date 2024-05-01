use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    config: Config,
}

impl State {
    pub fn new(config: Config) -> Self {
        State { config }
    }

    pub fn tick(&mut self) {}
}
