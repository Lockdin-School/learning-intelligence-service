use crate::configuration::state::AppState;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use uuid::Uuid;

#[get("/{student_id}/progress")]
pub async fn get_student_progress(
    state: Data<AppState>,
    student_id: Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let student_id = student_id.into_inner();

    log::info!(
        "progress.get.request | handler | get_student_progress | started | \"Getting student progress.\" | student_id={}",
        student_id
    );

    match state
        .progress_service
        .get_student_progress(student_id)
        .await
    {
        Ok(progress) => {
            log::info!(
                "progress.get.success | handler | get_student_progress | success | \"Student progress retrieved successfully.\" | student_id={}",
                student_id
            );

            Ok(HttpResponse::Ok().json(progress))
        }
        Err(error) => {
            log::error!(
                "progress.get.failure | handler | get_student_progress | failure | \"{:?}\" | student_id={}",
                error,
                student_id
            );

            Ok(HttpResponse::from_error(error))
        }
    }
}
