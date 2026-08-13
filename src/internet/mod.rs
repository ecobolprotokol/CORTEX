pub mod fetch;
pub mod parse;

use crate::config::InternetConfig;
use crate::error::Result;
use crate::types::*;

pub trait InternetInterface {
    fn fetch(&self, url: &str) -> Result<Observation>;
    fn enabled(&self) -> bool;
}

pub struct InternetInterfaceImpl {
    config: InternetConfig,
}

impl InternetInterfaceImpl {
    pub fn new(config: &InternetConfig) -> Result<Self> {
        Ok(Self { config: config.clone() })
    }
}

impl InternetInterface for InternetInterfaceImpl {
    fn fetch(&self, url: &str) -> Result<Observation> {
        if !self.config.enabled {
            return Err(crate::error::CortexError::SubsystemDisabled("Internet is disabled".into()));
        }
        Ok(Observation::from_internet("", url))
    }

    fn enabled(&self) -> bool {
        self.config.enabled
    }
}
