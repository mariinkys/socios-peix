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

use crate::core::models::{
    email::{Email, TodayEmail},
    interest::Interest,
};

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

#[server(SendInterestsEmail, "/api/emails/send-interests")]
pub async fn send_interests_email(
    interests: Vec<Interest>,
    subject: String,
    body: String,
) -> Result<(), ServerFnError> {
    use crate::core::email_client::EmailClient;

    if interests.is_empty() {
        return Err(ServerFnError::new("No interests provided"));
    }
    if subject.trim().is_empty() {
        return Err(ServerFnError::new("Subject cannot be empty"));
    }
    if body.trim().is_empty() {
        return Err(ServerFnError::new("Email body cannot be empty"));
    }

    let ext_email_client: Data<EmailClient> = extract().await?;
    let ext_database: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext_database.into_inner();
    let email_client: Arc<EmailClient> = ext_email_client.into_inner();

    let members = crate::core::models::member::Member::get_members_by_interests(&pool, interests)
        .await
        .map_err(|e| {
            leptos::logging::log!("Failed to retrieve members by interests: {}", e);
            ServerFnError::new("Failed to retrieve members with the provided interests")
        })?;

    if members.is_empty() {
        leptos::logging::log!("No members found with the provided interests");
        return Err(ServerFnError::new(
            "No members found with the provided interests",
        ));
    };

    let mut successful_sends = 0;
    let mut failed_sends = 0;

    for member in &members {
        match member.id {
            Some(member_id) => {
                match Email::send_single(
                    &pool,
                    &email_client,
                    member_id,
                    subject.clone(),
                    body.clone(),
                )
                .await
                {
                    Ok(()) => {
                        successful_sends += 1;
                    }
                    Err(_e) => {
                        failed_sends += 1;
                    }
                }
            }
            None => {
                failed_sends += 1;
                leptos::logging::log!("Member without ID found, skipping: {}", member);
            }
        }
    }

    if failed_sends > 0 && successful_sends == 0 {
        Err(ServerFnError::new("Failed to send emails to any members"))
    } else if failed_sends > 0 {
        Err(ServerFnError::new(format!(
            "Partially successful: sent {successful_sends}/{} emails",
            members.len()
        )))
    } else {
        Ok(())
    }
}

#[server(TestEmailConfig, "/api/emails/test")]
pub async fn test_email_config() -> Result<(), ServerFnError> {
    use crate::core::email_client::EmailClient;

    let ext_email_client: Data<EmailClient> = extract().await?;
    let email_client: Arc<EmailClient> = ext_email_client.into_inner();

    match email_client.mailer.test_connection() {
        Ok(val) => {
            if val {
                Ok(())
            } else {
                Err(ServerFnError::new(
                    "Failed to connect to email server, reason unknown",
                ))
            }
        }
        Err(e) => Err(ServerFnError::new(format!(
            "Failed to connect to email server {e}"
        ))),
    }
}

#[server(GetTodayEmails, "/api/today-emails")]
pub async fn get_today_emails() -> Result<Vec<TodayEmail>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let today = chrono::Local::now().naive_local().date();
    let result = Email::get_today_emails(&pool, today).await;

    match result {
        Ok(members) => Ok(members),
        Err(e) => {
            leptos::logging::log!("Failed to get today's birthday members: {}", e);
            Err(ServerFnError::new("Failed to retrieve birthday members"))
        }
    }
}
