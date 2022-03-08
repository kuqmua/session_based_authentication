use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );
    let _request_span_guard = request_span.enter();
    // `Result` has two variants: `Ok` and `Err`.
    // The first for successes, the second for failures.
    // We use a `match` statement to choose what to do based
    // on the outcome.
    // We will talk more about `Result` going forward!
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => {
            let request_span = tracing::info_span!(
                "New subscriber details have been saved",
                %request_id
            );
            let _request_span_guard = request_span.enter();
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            let request_span = tracing::error_span!(
                "Failed to execute query",
                %request_id,
                error = %e
            );
            let _request_span_guard = request_span.enter();
            HttpResponse::InternalServerError().finish()
        }
    }
}
