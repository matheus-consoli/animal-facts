use axum::{
    extract::{Query, State},
    http::status::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::{
    animal::{get_animal_info, get_random_animal_info, Animal},
    AppState,
};

#[derive(Debug, Serialize)]
pub struct Fact {
    animal: Animal,
    fact: String,
}

impl Fact {
    pub fn new(animal: Animal, fact: String) -> Self {
        Self { animal, fact }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryArgs {
    animal: Option<Animal>,
}

// #[tracing::instrument(skip_all)]
pub async fn fact(
    State(state): State<AppState>,
    Query(query): Query<QueryArgs>,
) -> Result<Json<Fact>, StatusCode> {
    let mut from_any = false;
    let (animal, animal_info) = if let Some(animal) = query.animal {
        (animal, get_animal_info(animal))
    } else {
        from_any = true;
        get_random_animal_info()
    };

    debug!(
        ?animal,
        from_any,
        url = animal_info.url(),
        "fetching animal resource"
    );
    let response = state
        .http_client
        .get(animal_info.url())
        .send()
        .await
        .map_err(|err| {
            error!(?err, "failed to get animal info");
            StatusCode::SERVICE_UNAVAILABLE
        })?;

    let fact = response.text().await.map_err(|err| {
        error!(?err, "failed to parse response text");
        StatusCode::SERVICE_UNAVAILABLE
    })?;
    let fact = animal_info.parse_into_fact(fact).map_err(|err| {
        error!(?err, "failed to parse to API response");
        StatusCode::SERVICE_UNAVAILABLE
    })?;

    Ok(Json(fact))
}
