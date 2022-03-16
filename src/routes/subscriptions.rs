use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::{PgPool, Transaction, Postgres};
use uuid::Uuid;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::ResponseError;

fn generate_subscription_token() -> String {
    let mut rng = thread_rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
            .map(char::from)
            .take(25)
            .collect()
  }

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// pub fn parse_subscriber(form: FormData) -> Result<NewSubscriber, String> {
//     let name = SubscriberName::parse(form.name)?;
//     let email = SubscriberEmail::parse(form.email)?;
//     Ok(NewSubscriber { email, name })
// }

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

// #[tracing::instrument(
//     name = "Adding a new subscriber",
//     skip(form, pool, email_client, base_url),
//     fields(
//         subscriber_email = %form.email,
//         subscriber_name = %form.name
//     )
// )]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> Result<HttpResponse, actix_web::Error> {
    //
    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return Ok(HttpResponse::BadRequest().finish()),
    };
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let subscriber_id = match insert_subscriber(&mut transaction, &new_subscriber).await {
        Ok(subscriber_id) => subscriber_id,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let subscription_token = generate_subscription_token();
    if store_token(&mut transaction, subscriber_id, &subscription_token)
    .await
    .is_err()
    {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    if transaction.commit().await.is_err() {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    if send_confirmation_email(
        &email_client, 
        new_subscriber, 
        &base_url.0,
        &subscription_token
    )
        .await
        .is_err()
    {
        return Ok(HttpResponse::InternalServerError().finish());
    }
    Ok(HttpResponse::Ok().finish())
}

pub struct StoreTokenError(sqlx::Error);

impl std::fmt::Debug for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nCaused by:\n\t{}", self, self.0)
    }
}

impl std::fmt::Display for StoreTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while \
            trying to store a subscription token."
        )
    }
}

impl std::error::Error for StoreTokenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // The compiler transparently casts `&sqlx::Error` into a `&dyn Error`
        Some(&self.0)
    }
}

impl ResponseError for StoreTokenError {}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[tracing::instrument(
    name = "Store subscription token in the database",
    skip(subscription_token, transaction)
)]
pub async fn store_token(
    transaction: &mut Transaction<'_, Postgres>,
    subscriber_id: Uuid,
    subscription_token: &str,
) -> Result<(), StoreTokenError> {
    sqlx::query!(
        r#"INSERT INTO subscription_tokens (subscription_token, subscriber_id)
        VALUES ($1, $2)"#,
        subscription_token,
        subscriber_id
    )
    .execute(transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        StoreTokenError(e)
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber, base_url)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
    subscription_token: &str
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!("{base_url}/subscriptions/confirm?subscription_token={subscription_token}");
    let plain_body = format!(
        "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
        confirmation_link
    );
    let html_body = format!(
        "Welcome to our newsletter!<br />\
        Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );
    email_client
        .send_email(new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
}

// /// Returns `true` if the input satisfies all our validation constraints
// /// on subscriber names, `false` otherwise.
// pub fn is_valid_name(s: &str) -> bool {
//     // `.trim()` returns a view over the input `s` without trailing
//     // whitespace-like characters.
//     // `.is_empty` checks if the view contains any character.
//     let is_empty_or_whitespace = s.trim().is_empty();

//     // A grapheme is defined by the Unicode standard as a "user-perceived"
//     // character: `å` is a single grapheme, but it is composed of two characters
//     // (`a` and `̊`).
//     //
//     // `graphemes` returns an iterator over the graphemes in the input `s`.
//     // `true` specifies that we want to use the extended grapheme definition set,
//     // the recommended one.
//     let is_too_long = s.graphemes(true).count() > 256;

//     // Iterate over all characters in the input `s` to check if any of them matches
//     // one of the characters in the forbidden array.
//     let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
//     let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

//     // Return `false` if any of our conditions have been violated
//     !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
// }

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, transaction)
)]
pub async fn insert_subscriber(
    transaction: &mut Transaction<'_, Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'pending_confirmation')"#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute( transaction)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(subscriber_id)
}
