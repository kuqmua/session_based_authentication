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

use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::routes::error_chain_fmt;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use sqlx::PgPool;

struct ConfirmedSubscriber {
    email: SubscriberEmail,
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
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
) -> Result<HttpResponse, PublishError> {
    let subscribers = get_confirmed_subscribers(&pool).await?;
    for subscriber in subscribers {
        match subscriber {
            Ok(subscriber) => {
                email_client
                    .send_email(
                        &subscriber.email,
                        &body.title,
                        &body.content.html,
                        &body.content.text,
                    )
                    .await
                    .with_context(|| {
                        format!("Failed to send newsletter issue to {}", subscriber.email)
                    })?;
            }
            Err(error) => {
                tracing::warn!(
                    // We record the error chain as a structured field
                    // on the log record.
                    error.cause_chain = ?error,
                    // Using `\` to split a long string literal over
                    // two lines, without creating a `\n` character.
                    "Skipping a confirmed subscriber. \
                    Their stored contact details are invalid",
                );
            }
        }
    }
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
async fn get_confirmed_subscribers(
    pool: &PgPool,
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let confirmed_subscribers = sqlx::query!(
        r#"
        SELECT email
        FROM subscriptions
        WHERE status = 'confirmed'
        "#,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| match SubscriberEmail::parse(r.email) {
        Ok(email) => Ok(ConfirmedSubscriber { email }),
        Err(error) => Err(anyhow::anyhow!(error)),
    })
    .collect();
    Ok(confirmed_subscribers)
}
