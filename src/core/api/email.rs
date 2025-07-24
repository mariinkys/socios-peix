// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "ssr")]
use std::sync::Arc;

use crate::core::models::email::Email;

#[server(SendSingleEmail, "/api/emails/send-single")]
pub async fn send_single_email(
    member_id: i32,
    subject: String,
    body: String,
) -> Result<(), ServerFnError> {
    use crate::core::email_client::EmailClient;

    let ext_email_client: Data<EmailClient> = extract().await?;
    let ext_database: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext_database.into_inner();
    let email_client: Arc<EmailClient> = ext_email_client.into_inner();

    let result = Email::send_single(&pool, &email_client, member_id, subject, body).await;

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            leptos::logging::log!("Failed to send single email: {}", e);
            Err(ServerFnError::new("Failed to send single email"))
        }
    }
}

#[server(MemberInterests, "/api/emails/member")]
pub async fn get_member_emails(member_id: i32) -> Result<Vec<Email>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Email::get_by_member(&pool, member_id).await;

    match result {
        Ok(emails) => Ok(emails),
        Err(e) => {
            leptos::logging::log!("Failed to get all emails: {}", e);
            Err(ServerFnError::new("Failed to retrieve all emails"))
        }
    }
}
