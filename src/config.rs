use std::collections::HashMap;

use eyre::Context;
use twelf::{config, Layer};

use crate::animal::Animal;

static CONFIG_FILE: &str = "./fact-settings.toml";

#[config]
pub struct Config {
    pub port: u16,
    pub addr: String,
    pub sources: HashMap<Animal, String>,
}

pub(crate) fn config() -> eyre::Result<Config> {
    let conf = Config::with_layers(&[Layer::Toml(CONFIG_FILE.into())]);

    conf.wrap_err_with(|| {
        format!(
        "something is wrong with the config file (`{CONFIG_FILE}`). {}",
        "Note that adding a new resource without editing the `Animal` enum will result in an error."
    )
    })
}
