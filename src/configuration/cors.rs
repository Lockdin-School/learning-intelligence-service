use actix_cors::Cors;
use actix_web::http::header;

pub fn build_cors(frontend_origin: &str, services: &str) -> Cors {
    Cors::default()
        .allowed_origin(frontend_origin)
        .allowed_origin(services)
        .allowed_methods(["GET", "POST", "DELETE", "PUT"])
        .allowed_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .supports_credentials()
        .max_age(3600)
}
