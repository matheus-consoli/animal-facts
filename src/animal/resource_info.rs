use std::marker::PhantomData;

use serde::de::DeserializeOwned;

use crate::fact::Fact;

pub trait AnimalResourceInfo: Send + Sync {
    fn url(&self) -> &str;
    fn parse_into_fact(&self, text: String) -> eyre::Result<Fact>;
}

#[inline(always)]
pub(super) fn make_animal_info<T>(url: String) -> Box<dyn AnimalResourceInfo>
where
    T: DeserializeOwned + Into<Fact> + Send + Sync + 'static,
{
    Box::new(make_animal_resource_info::<T>(url))
}

pub(super) fn make_animal_resource_info<T>(url: String) -> impl AnimalResourceInfo
where
    T: DeserializeOwned + Into<Fact> + Send + Sync + 'static,
{
    struct SomeAnimal<A: DeserializeOwned + Into<Fact>> {
        url: String,
        into: PhantomData<A>,
    }

    impl<A: DeserializeOwned + Into<Fact> + Send + Sync> AnimalResourceInfo for SomeAnimal<A> {
        fn url(&self) -> &str {
            &self.url
        }
        fn parse_into_fact(&self, text: String) -> eyre::Result<Fact> {
            Ok(serde_json::from_str::<A>(&text)?.into())
        }
    }

    SomeAnimal {
        url,
        into: PhantomData::<T>,
    }
}
