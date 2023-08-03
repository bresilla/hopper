#![allow(dead_code)]

use std::io;
use crate::places;
use crate::generation;
use crate::generation::Generation;
use crate::log::*;
use crate::{info, error, warning, note};
use crate::filesystem::*;
use crate::library::custom_error;
use crate::config;

// Constants
const DEFAULT_USER_GEN: &str =
"# --------------------- #
#    Generation File    #
# --------------------- #

# Packages to be installed via the native package manager.
pkgs = [
    # \"git\",
]

# Packages to be installed via Flatpak.
flatpaks = [
    # \"flatseal\",
]

# Flatpak repositories.
flatpak_repos = [
    # [\"flathub\", \"https://flathub.org/repo/flathub.flatpakrepo\"],
]
";

const DEFAULT_PACKAGE_MANAGER_CONFIG: &str =
"# ----------------------------------- #
#    Package Manager Configuration    #
# ----------------------------------- #

# Make sure to enter the exact command you use as the normal user!
# That means including 'sudo' or 'doas' or whatever if the command needs it.
# Where you would put packages, enter '#:?'.

# Example: install = \"sudo apt install #:?\"

install = \"\" # Example: sudo apt install #:?
remove = \"\" # Example: sudo apt remove #:?
sync = \"\" # Example: sudo apt update
upgrade = \"\" # Example: sudo apt upgrade

# ------------------------------- #
#    Additional configuration.    #
# ------------------------------- #

# many_pkg_args = BOOL: Can you supply many packages as an argument? Example: 'sudo apt install git vim wget'

config = { many_pkg_args = true }
";

// This determinds if a function should
// use the files from the user's config,
// or from the base() directory.
pub enum ConfigSide {
    User,
    System,
}

// What to grab a config file for.
pub enum Config {
    Generation,
}

// Create the user configuration.
pub fn init_user_config(force: bool) -> Result<(), io::Error> {
    if path_exists(places::base_user().as_str()) {
        if force == false {
            error!("The user configuration already exists, if you want to overwrite everything in your configuration, please use '--force'!");
            note!("Forcing to overwrite will overwrite EVERYTHING!!! So, if you are just trying to re-generate one broken file, copy everything you want to keep out of the directory first!");

            return Err(custom_error("Configuration already exists!"));
        }

        else {
            warning!("Overwriting existing configuration...");

            match remove_directory(places::base_user().as_str()) {
                Ok(_o) => info!("Removed directory: {}", places::base_user()),
                Err(e) => {
                    error!("Failed to remove directory: {}", places::base_user());
                    return Err(e);
                },
            };
        }
    }

    match create_directory(places::base_user().as_str()) {
        Ok(_o) => info!("Created directory: {}", places::base_user()),
        Err(e) => {
            error!("Failed to create directory: {}", places::base_user());
            return Err(e);
        },
    };

    let files = vec![
        (DEFAULT_USER_GEN, config::config_for(Config::Generation, ConfigSide::User)),
        (DEFAULT_PACKAGE_MANAGER_CONFIG, format!("{}/pkg_manager.toml", places::base_user())),
    ];

    for i in files.iter() {
        match write_file(i.0, i.1.as_str()) {
            Ok(_o) => info!("Created file: {}", i.1),
            Err(e) => {
                error!("Failed to create file: {}", i.1);
                return Err(e);
            },
        };
    }

    return Ok(());
}

// Return generation structure for...
pub fn gen(side: ConfigSide) -> Result<Generation, io::Error> {
    let generation: Generation = match toml::from_str(match read_file(config_for(Config::Generation, side).as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to read generation TOML file!");
            return Err(e);
        },
    }.as_str()) {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to parse generation TOML file!");
            error!("TOML Error: {:?}", e);

            return Err(custom_error("Failed to parse TOML file!"));
        },
    };

    return Ok(generation);
}

// Return path for a config file.
pub fn config_for(config: Config, side: ConfigSide) -> String {
    return match config {
        Config::Generation => match side {
            ConfigSide::User => format!("{}/gen.toml", places::base_user()),
            ConfigSide::System => generation::current_gen(),
        },
    };
}
