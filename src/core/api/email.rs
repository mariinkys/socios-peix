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

// Helper function to extract common dependencies
#[cfg(feature = "ssr")]
async fn extract_dependencies() -> Result<
    (
        Arc<Pool<Sqlite>>,
        Arc<crate::core::email_client::EmailClient>,
    ),
    ServerFnError,
> {
    let ext_email_client: Data<crate::core::email_client::EmailClient> = extract().await?;
    let ext_database: Data<Pool<Sqlite>> = extract().await?;
    let pool = ext_database.into_inner();
    let email_client = ext_email_client.into_inner();
    Ok((pool, email_client))
}

// Helper function for input validation
#[cfg(feature = "ssr")]
fn validate_email_inputs(subject: &str, body: &str) -> Result<(), ServerFnError> {
    if subject.trim().is_empty() {
        return Err(ServerFnError::new("Subject cannot be empty"));
    }
    if body.trim().is_empty() {
        return Err(ServerFnError::new("Email body cannot be empty"));
    }
    Ok(())
}

// Helper function to handle bulk email sending results
#[cfg(feature = "ssr")]
fn handle_bulk_results(
    successful: usize,
    failed: usize,
    total: usize,
) -> Result<(), ServerFnError> {
    match (successful, failed) {
        (0, _) if failed > 0 => Err(ServerFnError::new("Failed to send emails to any members")),
        (_, f) if f > 0 => Err(ServerFnError::new(format!(
            "Partially successful: sent {successful}/{total} emails"
        ))),
        _ => Ok(()),
    }
}

// Helper function to send emails to a list of members
#[cfg(feature = "ssr")]
async fn send_to_members(
    pool: &Pool<Sqlite>,
    email_client: &crate::core::email_client::EmailClient,
    members: &[crate::core::models::member::Member],
    subject: String,
    body: String,
    original_body: String,
    require_email: bool,
) -> Result<(), ServerFnError> {
    let mut successful_sends = 0;
    let mut failed_sends = 0;

    for member in members {
        if let Some(member_id) = member.id {
            if !require_email || !member.email.trim().is_empty() {
                match Email::send_single(
                    pool,
                    email_client,
                    member_id,
                    subject.clone(),
                    body.clone(),
                    original_body.clone(),
                )
                .await
                {
                    Ok(()) => successful_sends += 1,
                    Err(_) => failed_sends += 1,
                }
            }
        } else {
            failed_sends += 1;
            leptos::logging::log!("Member without ID found, skipping: {}", member);
        }
    }

    handle_bulk_results(successful_sends, failed_sends, members.len())
}

#[server(SendSingleEmail, "/api/emails/send-single")]
pub async fn send_single_email(
    member_id: i32,
    subject: String,
    original_body: String,
) -> Result<(), ServerFnError> {
    use crate::core::utils::email::EmailKind;

    let (pool, email_client) = extract_dependencies().await?;

    validate_email_inputs(&subject, &original_body)?;
    let body =
        EmailKind::get_styled(&EmailKind::Normal, Some(original_body.clone()), None, None)
            .map_err(|err| ServerFnError::new(format!("Error creating email template: {err}")))?;

    Email::send_single(
        &pool,
        &email_client,
        member_id,
        subject,
        body,
        original_body,
    )
    .await
    .map_err(|e| {
        leptos::logging::log!("Failed to send single email: {}", e);
        ServerFnError::new("Failed to send single email")
    })
}

#[server(SendCuponEmail, "/api/emails/send-cupon")]
pub async fn send_cupon_email(
    member_id: i32,
    cupon_description: String,
    cupon_expires_at: Option<chrono::NaiveDate>,
    subject: String,
    original_body: String,
) -> Result<(), ServerFnError> {
    use crate::core::models::{cupon::Cupon, member::Member};
    use crate::core::utils::{cupon::generate_random_code, email::EmailKind};

    let (pool, email_client) = extract_dependencies().await?;
    if subject.trim().is_empty() {
        return Err(ServerFnError::new("Subject cannot be empty"));
    }
    if cupon_description.trim().is_empty() {
        return Err(ServerFnError::new("Cupon description cannot be empty"));
    }
    if cupon_expires_at.is_none() {
        return Err(ServerFnError::new("Cupon description cannot be empty"));
    }

    let member = Member::get_single(&pool, member_id)
        .await
        .map_err(|err| ServerFnError::new(format!("Error getting member: {err}")))?;

    let cupon = Cupon {
        member_id: Some(member_id),
        code: generate_random_code(),
        description: cupon_description,
        expires_at: cupon_expires_at,
        ..Default::default()
    };

    Cupon::add(&pool, cupon.clone())
        .await
        .map_err(|err| ServerFnError::new(format!("Error creating cupon: {err}")))?;

    let body = EmailKind::get_styled(
        &EmailKind::Cupon,
        Some(original_body.clone()),
        Some(member),
        Some(cupon),
    )
    .map_err(|err| ServerFnError::new(format!("Error creating email template: {err}")))?;

    Email::send_single(
        &pool,
        &email_client,
        member_id,
        subject,
        body,
        original_body,
    )
    .await
    .map_err(|e| {
        leptos::logging::log!("Failed to send single email: {}", e);
        ServerFnError::new("Failed to send single email")
    })
}

#[server(MemberInterests, "/api/emails/member")]
pub async fn get_member_emails(member_id: i32) -> Result<Vec<Email>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool = ext.into_inner();

    Email::get_by_member(&pool, member_id).await.map_err(|e| {
        leptos::logging::log!("Failed to get all emails: {}", e);
        ServerFnError::new("Failed to retrieve all emails")
    })
}

#[server(SendInterestsEmail, "/api/emails/send-interests")]
pub async fn send_interests_email(
    interests: Vec<Interest>,
    subject: String,
    original_body: String,
) -> Result<(), ServerFnError> {
    use crate::core::utils::email::EmailKind;

    if interests.is_empty() {
        return Err(ServerFnError::new("No interests provided"));
    }
    validate_email_inputs(&subject, &original_body)?;

    let (pool, email_client) = extract_dependencies().await?;

    let members = crate::core::models::member::Member::get_members_by_interests(&pool, interests)
        .await
        .map_err(|e| {
            leptos::logging::log!("Failed to retrieve members by interests: {}", e);
            ServerFnError::new("Failed to retrieve members with the provided interests")
        })?;

    if members.is_empty() {
        return Err(ServerFnError::new(
            "No members found with the provided interests",
        ));
    }

    let body =
        EmailKind::get_styled(&EmailKind::Normal, Some(original_body.clone()), None, None)
            .map_err(|err| ServerFnError::new(format!("Error creating email template: {err}")))?;

    send_to_members(
        &pool,
        &email_client,
        &members,
        subject,
        body,
        original_body,
        false,
    )
    .await
}

#[server(SendAllEmail, "/api/emails/send-all")]
pub async fn send_all_email(subject: String, original_body: String) -> Result<(), ServerFnError> {
    use crate::core::utils::email::EmailKind;

    validate_email_inputs(&subject, &original_body)?;

    let (pool, email_client) = extract_dependencies().await?;

    let members = crate::core::models::member::Member::get_all(&pool)
        .await
        .map_err(|e| {
            leptos::logging::log!("Failed to retrieve members: {}", e);
            ServerFnError::new("Failed to retrieve members")
        })?;

    if members.is_empty() {
        return Err(ServerFnError::new("No members found"));
    }

    let body =
        EmailKind::get_styled(&EmailKind::Normal, Some(original_body.clone()), None, None)
            .map_err(|err| ServerFnError::new(format!("Error creating email template: {err}")))?;

    send_to_members(
        &pool,
        &email_client,
        &members,
        subject,
        body,
        original_body,
        true,
    )
    .await
}

#[server(TestEmailConfig, "/api/emails/test")]
pub async fn test_email_config() -> Result<(), ServerFnError> {
    let ext_email_client: Data<crate::core::email_client::EmailClient> = extract().await?;
    let email_client = ext_email_client.into_inner();

    match email_client.mailer.test_connection() {
        Ok(true) => Ok(()),
        Ok(false) => Err(ServerFnError::new(
            "Failed to connect to email server, reason unknown",
        )),
        Err(e) => Err(ServerFnError::new(format!(
            "Failed to connect to email server {e}"
        ))),
    }
}

#[server(GetTodayEmails, "/api/today-emails")]
pub async fn get_today_emails() -> Result<Vec<TodayEmail>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool = ext.into_inner();

    let today = chrono::Local::now().naive_local().date();

    Email::get_today_emails(&pool, today).await.map_err(|e| {
        leptos::logging::log!("Failed to get today's birthday members: {}", e);
        ServerFnError::new("Failed to retrieve birthday members")
    })
}

#[server(prefix = "/api", endpoint = "emails/send-birthday")]
pub async fn send_birthday_emails() -> Result<(), ServerFnError> {
    use crate::core::utils::email::EmailKind;

    leptos::logging::log!("Starting birthday email sending process");

    let (pool, email_client) = extract_dependencies().await?;
    let today = chrono::Local::now().naive_local().date();

    let members = crate::core::models::member::Member::get_today_members(&pool, today)
        .await
        .map_err(|e| {
            leptos::logging::log!(
                "DATABASE ERROR: Failed to get today's birthday members: {}",
                e
            );
            ServerFnError::new("Failed to retrieve birthday members")
        })?;

    if members.is_empty() {
        leptos::logging::log!("No members have birthdays today ({})", today);
        return Ok(());
    }

    let mut successful_sends = 0;
    let mut failed_sends = 0;

    for member in &members {
        let body =
            match EmailKind::get_styled(&EmailKind::Birthday, None, Some(member.clone()), None) {
                Ok(body_content) => body_content,
                Err(err) => {
                    leptos::logging::log!(
                        "Failed to generate email body for member {}: {}",
                        member.id.unwrap_or(0),
                        err
                    );
                    failed_sends += 1;
                    continue;
                }
            };

        if let Some(member_id) = member.id {
            match Email::send_single(
                &pool,
                &email_client,
                member_id,
                "Feliz Cumpleaños!".to_string(),
                body,
                "Email de Cumpleaños por defecto.".to_string(),
            )
            .await
            {
                Ok(()) => successful_sends += 1,
                Err(e) => {
                    failed_sends += 1;
                    leptos::logging::log!(
                        "Failed to send birthday email to member ID: {} ({}): {}",
                        member_id,
                        member.name,
                        e
                    );
                }
            }
        } else {
            failed_sends += 1;
        }
    }

    match (successful_sends, failed_sends) {
        (0, _) if failed_sends > 0 => {
            leptos::logging::log!(
                "COMPLETE FAILURE: Failed to send emails to any of the {} members",
                members.len()
            );
            Err(ServerFnError::new(format!(
                "Failed to send emails to any of the {} members",
                members.len()
            )))
        }
        (_, f) if f > 0 => {
            leptos::logging::log!(
                "PARTIAL SUCCESS: sent {}/{} emails successfully, {} failed",
                successful_sends,
                members.len(),
                failed_sends
            );
            Err(ServerFnError::new(format!(
                "Partially successful: sent {}/{} emails successfully, {} failed",
                successful_sends,
                members.len(),
                failed_sends
            )))
        }
        _ => {
            leptos::logging::log!(
                "COMPLETE SUCCESS: Successfully sent birthday emails to all {} members",
                successful_sends
            );
            Ok(())
        }
    }
}

// Helper function for coupon email sending
#[allow(clippy::too_many_arguments)]
#[cfg(feature = "ssr")]
async fn send_coupon_emails_to_members(
    pool: &Pool<Sqlite>,
    email_client: &crate::core::email_client::EmailClient,
    members: &[crate::core::models::member::Member],
    subject: String,
    original_body: String,
    cupon_description: String,
    cupon_expires_at: Option<chrono::NaiveDate>,
) -> Result<(), ServerFnError> {
    use crate::core::models::cupon::Cupon;
    use crate::core::utils::{cupon::generate_random_code, email::EmailKind};

    if subject.trim().is_empty() {
        return Err(ServerFnError::new("Subject cannot be empty"));
    }
    if cupon_description.trim().is_empty() {
        return Err(ServerFnError::new("Cupon description cannot be empty"));
    }
    if cupon_expires_at.is_none() {
        return Err(ServerFnError::new("Cupon description cannot be empty"));
    }

    let mut successful_sends = 0;
    let mut failed_sends = 0;
    let cupon_code = generate_random_code();

    for member in members {
        if let Some(member_id) = member.id {
            if !member.email.trim().is_empty() {
                let cupon = Cupon {
                    member_id: Some(member_id),
                    code: cupon_code.clone(),
                    description: cupon_description.clone(),
                    expires_at: cupon_expires_at,
                    ..Default::default()
                };

                if Cupon::add(pool, cupon.clone()).await.is_err() {
                    failed_sends += 1;
                    continue;
                }

                let body = match EmailKind::get_styled(
                    &EmailKind::Cupon,
                    Some(original_body.clone()),
                    Some(member.clone()),
                    Some(cupon),
                ) {
                    Ok(body_content) => body_content,
                    Err(_) => {
                        failed_sends += 1;
                        continue;
                    }
                };

                match Email::send_single(
                    pool,
                    email_client,
                    member_id,
                    subject.clone(),
                    body,
                    original_body.clone(),
                )
                .await
                {
                    Ok(()) => successful_sends += 1,
                    Err(_) => failed_sends += 1,
                }
            }
        } else {
            failed_sends += 1;
            leptos::logging::log!("Member without ID found, skipping: {}", member);
        }
    }

    handle_bulk_results(successful_sends, failed_sends, members.len())
}

#[server(SendInterestsCuponEmail, "/api/emails/send-interests-cupon")]
pub async fn send_interests_cupon_email(
    interests: Vec<Interest>,
    cupon_description: String,
    cupon_expires_at: Option<chrono::NaiveDate>,
    subject: String,
    original_body: String,
) -> Result<(), ServerFnError> {
    if interests.is_empty() {
        return Err(ServerFnError::new("No interests provided"));
    }
    if subject.trim().is_empty() {
        return Err(ServerFnError::new("Subject cannot be empty"));
    }
    if cupon_expires_at.is_none() {
        return Err(ServerFnError::new("Cupon expiration cannot be empty"));
    }
    if cupon_description.trim().is_empty() {
        return Err(ServerFnError::new("Cupon description cannot be empty"));
    }

    let (pool, email_client) = extract_dependencies().await?;

    let members = crate::core::models::member::Member::get_members_by_interests(&pool, interests)
        .await
        .map_err(|e| {
            leptos::logging::log!("Failed to retrieve members by interests: {}", e);
            ServerFnError::new("Failed to retrieve members with the provided interests")
        })?;

    if members.is_empty() {
        return Err(ServerFnError::new(
            "No members found with the provided interests",
        ));
    }

    send_coupon_emails_to_members(
        &pool,
        &email_client,
        &members,
        subject,
        original_body,
        cupon_description,
        cupon_expires_at,
    )
    .await
}

#[server(SendAllCuponEmail, "/api/emails/send-all-cupon")]
pub async fn send_all_cupon_email(
    cupon_description: String,
    cupon_expires_at: Option<chrono::NaiveDate>,
    subject: String,
    original_body: String,
) -> Result<(), ServerFnError> {
    if subject.trim().is_empty() {
        return Err(ServerFnError::new("Subject cannot be empty"));
    }
    if cupon_expires_at.is_none() {
        return Err(ServerFnError::new("Cupon expiration cannot be empty"));
    }
    if cupon_description.trim().is_empty() {
        return Err(ServerFnError::new("Cupon description cannot be empty"));
    }

    let (pool, email_client) = extract_dependencies().await?;

    let members = crate::core::models::member::Member::get_all(&pool)
        .await
        .map_err(|e| {
            leptos::logging::log!("Failed to retrieve members: {}", e);
            ServerFnError::new("Failed to retrieve members")
        })?;

    if members.is_empty() {
        return Err(ServerFnError::new("No members found"));
    }

    send_coupon_emails_to_members(
        &pool,
        &email_client,
        &members,
        subject,
        original_body,
        cupon_description,
        cupon_expires_at,
    )
    .await
}
