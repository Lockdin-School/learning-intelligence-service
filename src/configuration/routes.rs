use actix_web::web;
use crate::core::observation::api::handlers::{create_observation, get_observation_by_id, observation_exists_by_source_event};

pub fn configure(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        // ------------- configure routes ------------
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
            )
            .service(
                web::scope("/observations")
                    .service(create_observation)
                    .service(get_observation_by_id)
                    .service(observation_exists_by_source_event)
            ),
    );
}
