// use crate::authentication::AuthError;
// use actix_web::{web, HttpResponse};
// use secrecy::Secret;

// #[tracing::instrument(
//     name = "Publish a newsletter issue",
//     skip(body, pool, email_client, request),
//     fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
// )]
// pub async fn publish_newsletter(
//     body: web::Json<BodyData>,
//     pool: web::Data<PgPool>,
//     email_client: web::Data<EmailClient>,
//     request: HttpRequest,
// ) -> Result<HttpResponse, PublishError> {
//     let credentials = basic_authentication(request.headers()).map_err(PublishError::AuthError)?;
//     tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
//     let user_id = validate_credentials(credentials, &pool).await?;
//     tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
// }
// ////////
// async fn validate_credentials(
//     credentials: Credentials,
//     pool: &PgPool,
//     // We are returning a `PublishError`,
//     // which is a specific error type detailing
//     // the relevant failure modes of `POST /newsletters`
//     // (not just auth!)
// ) -> Result<uuid::Uuid, Result<uuid::Uuid, AuthError>> {
//     let mut user_id = None;
//     let mut expected_password_hash = Secret::new(
//         "$argon2id$v=19$m=15000,t=2,p=1$\
//         gZiV/M1gPc22ElAH/Jh1Hw$\
//         CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
//             .to_string(),
//     );

//     if let Some((stored_user_id, stored_password_hash)) =
//         get_stored_credentials(&credentials.username, pool)
//             .await
//             .map_err(PublishError::UnexpectedError)?
//     {
//         user_id = Some(stored_user_id);
//         expected_password_hash = stored_password_hash;
//     }

//     spawn_blocking_with_tracing(move || {
//         verify_password_hash(expected_password_hash, credentials.password)
//     })
//     .await
//     .context("Failed to spawn blocking task.")
//     .map_err(PublishError::UnexpectedError)??;

//     user_id.ok_or_else(|| PublishError::AuthError(anyhow::anyhow!("Unknown username.")))
// }

use crate::routes::error_chain_fmt;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

struct ConfirmedSubscriber {
    email: String,
}

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for PublishError {
    fn status_code(&self) -> StatusCode {
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    html: String,
    text: String,
}

// Dummy implementation
pub async fn publish_newsletter(
    _body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PublishError> {
    let subscribers = get_confirmed_subscribers(&pool).await?;
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<ConfirmedSubscriber>, anyhow::Error> {
    let rows = sqlx::query_as!(
        ConfirmedSubscriber,
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
