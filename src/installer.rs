pub mod config;

use config::Config;

#[derive(Debug)]
pub struct Installer {
    config: Config
}

impl Installer {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn install(&self) {
        // TODO: impl install
    }
}
