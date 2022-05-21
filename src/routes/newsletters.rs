use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use crate::routes::subscriptions::error_chain_fmt;
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse, ResponseError};
use anyhow::Context;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(thiserror::Error)]
pub enum PublishError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for PublishError {
    fn status_code(&self) -> StatusCode {
        match self {
            PublishError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for PublishError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Deserialize)]
struct Content {
    text: String,
    html: String,
}
#[derive(Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[tracing::instrument(name = "Publishing newsletter", skip(pool, email_client, body))]
#[post("/newsletters")]
pub async fn publish_newsletter(
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    body: web::Json<BodyData>,
) -> Result<HttpResponse, PublishError> {
    let confirmed_subscribers = get_confirmed_subscribers(&pool).await?;

    for subscriber in confirmed_subscribers {
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
                        format!("Failed to send newsletter issue to {}", &subscriber.email)
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

#[derive(Debug)]
pub struct ConfirmedSubscriber {
    email: SubscriberEmail,
}

#[tracing::instrument(name = "Get confirmed subscribers", skip(pool))]
pub async fn get_confirmed_subscribers(
    pool: &PgPool,
    // We are returning a `Vec` of `Result`s in the happy case.
    // This allows the caller to bubble up errors due to network issues or other
    // transient failures using the `?` operator, while the compiler
    // forces them to handle the subtler mapping error.
    // See http://sled.rs/errors.html for a deep-dive about this technique.
) -> Result<Vec<Result<ConfirmedSubscriber, anyhow::Error>>, anyhow::Error> {
    let rows = sqlx::query!("SELECT email FROM subscriptions WHERE status = 'confirmed'")
        .fetch_all(pool)
        .await?;

    Ok(rows
        .into_iter()
        .map(|r| match SubscriberEmail::parse(r.email) {
            Ok(email) => Ok(ConfirmedSubscriber { email }),
            Err(error) => Err(anyhow::anyhow!(error)),
        })
        .collect())
}
