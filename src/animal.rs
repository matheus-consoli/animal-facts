mod resource_info;
mod util;

use std::{collections::HashMap, sync::OnceLock};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use variant_count::VariantCount;

use crate::fact::Fact;

pub(crate) use self::util::{get_animal_info, get_random_animal_info};
use self::{
    resource_info::{make_animal_info, AnimalResourceInfo},
    util::try_animal_from_config,
};

// TODO: remove `VariantCount` once `core::mem::variant` gets stabilized.
// Tracking issue: github.com/rust-lang/rust/issues/73662
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, VariantCount)]
#[serde(rename_all = "lowercase")]
pub enum Animal {
    Cat,
    Dog,
}

static ANIMAL_INDEX: OnceLock<IndexMap<Animal, Box<dyn AnimalResourceInfo>>> = OnceLock::new();

#[inline]
pub fn init_animal_index(config: HashMap<Animal, String>) -> eyre::Result<()> {
    eyre::ensure!(
        Animal::VARIANT_COUNT == config.len(),
        "animal count doesn't match with the config - compare the `[sources]` from the config with the `Animal` enum"
    );

    let index = IndexMap::from([
        (
            Animal::Cat,
            make_animal_info::<CatFact>(try_animal_from_config(&config, Animal::Cat)?),
        ),
        (
            Animal::Dog,
            make_animal_info::<DogFact>(try_animal_from_config(&config, Animal::Dog)?),
        ),
    ]);

    eyre::ensure!(
        Animal::VARIANT_COUNT == index.len(),
        "animal count and animal index doesn't match. have you forgotten to set it?"
    );

    ANIMAL_INDEX
        .set(index)
        .map_err(|_| "already set")
        .expect("static var `ANIMAL_INDEX` should be set only once");
    Ok(())
}

#[derive(Deserialize)]
struct CatFact {
    text: String,
}

impl From<CatFact> for Fact {
    fn from(cat: CatFact) -> Fact {
        Fact::new(Animal::Cat, cat.text)
    }
}

#[derive(Deserialize)]
struct DogFact {
    facts: Vec<String>,
}

impl From<DogFact> for Fact {
    fn from(mut dog: DogFact) -> Fact {
        Fact::new(Animal::Dog, dog.facts.pop().unwrap())
    }
}
