#![allow(dead_code)]

use std::collections::HashMap;
use std::io;
use serde::Deserialize;
use crate::library::*;
use crate::log::*;
use crate::{info, error};

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields, default)]
pub struct PackageManager {
    pub install: String,
    pub remove: String,
    pub sync: String,
    pub upgrade: String,
    pub config: HashMap<String, bool>,
    pub plural_name: String,
}

impl PackageManager {
    pub fn config_value(&self, key: &str) -> Result<bool, io::Error> {
        return match self.config.get(key) {
            Some(s) => Ok(*s),
            None => {
                error!("Missing keyword in package manager configuration! ({})", key);
                return Err(custom_error("Missing keyword in package manager config!"));
            },
        };
    }

    pub fn install(&self, pkgs: &str) -> Result<(), io::Error> {
        match run_command(sed(self.install.as_str(), "#:?", pkgs).as_str()) {
            true => info!("Successfully installed {}!", self.plural_name),
            false => {
                error!("Failed to install {}!", self.plural_name);

                return Err(custom_error(format!("Failed to install {}!", self.plural_name).as_str()));
            },
        };

        return Ok(());
    }

    pub fn remove(&self, pkgs: &str) -> Result<(), io::Error> {
        match run_command(sed(self.remove.as_str(), "#:?", pkgs).as_str()) {
            true => info!("Successfully removed {}!", self.plural_name),
            false => {
                error!("Failed to remove {}!", self.plural_name);

                return Err(custom_error(format!("Failed to remove {}!", self.plural_name).as_str()));
            },
        };

        return Ok(());
    }

    pub fn sync(&self) -> Result<(), io::Error> {
        match run_command(self.sync.as_str()) {
            true => info!("Successfully installed repositories!"),
            false => {
                error!("Failed to sync repositories!");

                return Err(custom_error("Failed to sync repositories!"));
            },
        };

        return Ok(());
    }

    pub fn upgrade(&self) -> Result<(), io::Error> {
        match run_command(self.upgrade.as_str()) {
            true => info!("Successfully upgraded {}!", self.plural_name),
            false => {
                error!("Failed to upgrade {}!", self.plural_name);

                return Err(custom_error(format!("Failed to upgrade {}!", self.plural_name).as_str()));
            },
        };

        return Ok(());
    }

    pub fn set_plural_name(&mut self, pn: &str) {
        self.plural_name = pn.to_string();
    }

    fn return_config_hashmap_default() -> HashMap<String, bool> {
        let mut hm: HashMap<String, bool> = HashMap::new();

        hm.insert("many_pkg_args".to_string(), true);

        return hm;
    }

    pub fn default() -> Self {
        return Self {
            install: String::new(),
            remove: String::new(),
            sync: String::new(),
            upgrade: String::new(),
            config: Self::return_config_hashmap_default(),
            plural_name: String::from("PACKAGES FROM UNKNOWN PACKAGE MANAGER"),
        };
    }
}
