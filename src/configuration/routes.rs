use crate::core::observations::api::handlers::{
    create_observation, get_observation_by_id, get_observations_by_student_id,
    observation_exists_by_source_event,
};
use crate::core::progress::api::handlers::get_student_progress;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        // ------------- configure routes ------------
        web::scope("/api/v1")
            .service(web::scope("/auth"))
            .service(
                web::scope("/observations")
                    .service(create_observation)
                    .service(get_observation_by_id)
                    .service(observation_exists_by_source_event),
            )
            .service(
                web::scope("/students")
                    .service(get_observations_by_student_id)
                    .service(get_student_progress),
            ),
    );
}
