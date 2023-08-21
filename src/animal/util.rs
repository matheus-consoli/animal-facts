use std::collections::HashMap;

use super::{resource_info::AnimalResourceInfo, Animal, ANIMAL_INDEX};

pub(super) fn try_animal_from_config(
    config: &HashMap<Animal, String>,
    animal: Animal,
) -> eyre::Result<String> {
    config
        .get(&animal)
        .ok_or_else(|| eyre::eyre!("`{animal:?}` source not found in the config file"))
        .map(String::from)
}

pub fn get_animal_info(animal: Animal) -> &'static dyn AnimalResourceInfo {
    ANIMAL_INDEX.get().unwrap().get(&animal).unwrap().as_ref()
}

pub fn get_random_animal_info() -> (Animal, &'static dyn AnimalResourceInfo) {
    let size = ANIMAL_INDEX.get().unwrap().len();
    let random_index = rand::random::<usize>() % size;
    ANIMAL_INDEX
        .get()
        .unwrap()
        .get_index(random_index)
        .map(|(&k, v)| (k, v.as_ref()))
        .unwrap()
}
