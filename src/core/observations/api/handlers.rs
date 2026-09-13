use crate::configuration::state::AppState;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, get, post};
use uuid::Uuid;
use crate::core::observations::models::Observation::ObservationNew;

#[post("")]
pub async fn create_observation(
    state: Data<AppState>,
    payload: Json<ObservationNew>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "observation.create.request | handler | create_observation | started | \"Creating observation.\" |"
    );

    match state
        .observation_service
        .create(&payload.into_inner())
        .await
    {
        Ok(observation) => {
            log::info!(
                "observation.create.success | handler | create_observation | success | \"Observation created successfully.\" | observation_id={}",
                observation.id
            );

            Ok(HttpResponse::Ok().json(observation))
        }

        Err(error) => {
            log::error!(
                "observation.create.failure | handler | create_observation | failure | \"{:?}\" |",
                error
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

#[get("/{id}")]
pub async fn get_observation_by_id(
    state: Data<AppState>,
    id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let observation_id = id.into_inner();

    log::info!(
        "observation.get.request | handler | get_observation_by_id | started | \"Getting observation by id.\" | observation_id={}",
        observation_id
    );

    match state
        .observation_service
        .find_by_id(observation_id)
        .await
    {
        Ok(observation) => {
            log::info!(
                "observation.get.success | handler | get_observation_by_id | success | \"Observation found.\" | observation_id={}",
                observation_id
            );

            Ok(HttpResponse::Ok().json(observation))
        }

        Err(error) => {
            log::error!(
                "observation.get.failure | handler | get_observation_by_id | failure | \"{:?}\" | observation_id={}",
                error,
                observation_id
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}

#[get("/source/{source_service}/{source_event_id}")]
pub async fn observation_exists_by_source_event(
    state: Data<AppState>,
    path: Path<(String, Uuid)>,
) -> actix_web::Result<HttpResponse> {
    let (source_service, source_event_id) = path.into_inner();

    log::info!(
        "observation.exists.request | handler | observation_exists_by_source_event | started | \"Checking observation source event.\" | source_service={} | source_event_id={}",
        source_service,
        source_event_id
    );

    match state
        .observation_service
        .exists_by_source_event(&source_service, source_event_id)
        .await
    {
        Ok(exists) => {
            log::info!(
                "observation.exists.success | handler | observation_exists_by_source_event | success | \"Observation source event checked.\" | exists={} | source_service={} | source_event_id={}",
                exists,
                source_service,
                source_event_id
            );

            Ok(HttpResponse::Ok().json(exists))
        }

        Err(error) => {
            log::error!(
                "observation.exists.failure | handler | observation_exists_by_source_event | failure | \"{:?}\" | source_service={} | source_event_id={}",
                error,
                source_service,
                source_event_id
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}