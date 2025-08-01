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
    original_body: String,
) -> Result<(), ServerFnError> {
    use crate::core::email_client::EmailClient;
    use crate::core::utils::email::EmailKind;

    let ext_email_client: Data<EmailClient> = extract().await?;
    let ext_database: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext_database.into_inner();
    let email_client: Arc<EmailClient> = ext_email_client.into_inner();

    let body = EmailKind::get_styled(&EmailKind::Normal, Some(original_body.clone()), None, None);
    if let Err(err) = body {
        return Err(ServerFnError::new(format!(
            "Error creating email template: {err}"
        )));
    }

    let result = Email::send_single(
        &pool,
        &email_client,
        member_id,
        subject,
        body.unwrap(),
        original_body,
    )
    .await;

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            leptos::logging::log!("Failed to send single email: {}", e);
            Err(ServerFnError::new("Failed to send single email"))
        }
    }
}

#[server(SendCuponEmail, "/api/emails/send-cupon")]
pub async fn send_cupon_email(
    member_id: i32,
    cupon_description: String,
    cupon_expires_at: Option<chrono::NaiveDate>,
    subject: String,
    original_body: String,
) -> Result<(), ServerFnError> {
    use crate::core::email_client::EmailClient;
    use crate::core::models::cupon::Cupon;
    use crate::core::models::member::Member;
    use crate::core::utils::cupon::generate_random_code;
    use crate::core::utils::email::EmailKind;

    let ext_email_client: Data<EmailClient> = extract().await?;
    let ext_database: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext_database.into_inner();
    let email_client: Arc<EmailClient> = ext_email_client.into_inner();

    let member = Member::get_single(&pool, member_id).await;
    let member = if let Err(err) = member {
        return Err(ServerFnError::new(format!("Error getting member: {err}")));
    } else {
        member.unwrap()
    };

    let cupon = Cupon {
        member_id: Some(member_id),
        code: generate_random_code(),
        description: cupon_description,
        expires_at: cupon_expires_at,
        ..Default::default()
    };
    let cupon_result = Cupon::add(&pool, cupon.clone()).await;
    if let Err(err) = cupon_result {
        return Err(ServerFnError::new(format!("Error creating cupon: {err}")));
    };

    let body = EmailKind::get_styled(
        &EmailKind::Cupon,
        Some(original_body.clone()),
        Some(member),
        Some(cupon),
    );
    if let Err(err) = body {
        return Err(ServerFnError::new(format!(
            "Error creating email template: {err}"
        )));
    }

    let result = Email::send_single(
        &pool,
        &email_client,
        member_id,
        subject,
        body.unwrap(),
        original_body,
    )
    .await;

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
    original_body: String,
) -> Result<(), ServerFnError> {
    use crate::core::email_client::EmailClient;
    use crate::core::utils::email::EmailKind;

    if interests.is_empty() {
        return Err(ServerFnError::new("No interests provided"));
    }
    if subject.trim().is_empty() {
        return Err(ServerFnError::new("Subject cannot be empty"));
    }
    if original_body.trim().is_empty() {
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

    let body = EmailKind::get_styled(&EmailKind::Normal, Some(original_body.clone()), None, None);
    let body = if let Err(err) = body {
        return Err(ServerFnError::new(format!(
            "Error creating email template: {err}"
        )));
    } else {
        body.unwrap()
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
                    original_body.clone(),
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

#[server(prefix = "/api", endpoint = "emails/send-birthday")]
pub async fn send_birthday_emails() -> Result<(), ServerFnError> {
    use crate::core::email_client::EmailClient;
    use crate::core::models::email::Email;
    use crate::core::utils::email::EmailKind;

    leptos::logging::log!("Starting birthday email sending process");

    let ext_email_client: Data<EmailClient> = extract().await?;
    let ext_database: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext_database.into_inner();
    let email_client: Arc<EmailClient> = ext_email_client.into_inner();

    let today = chrono::Local::now().naive_local().date();

    let members_result = crate::core::models::member::Member::get_today_members(&pool, today).await;

    match members_result {
        Ok(members) => {
            if !members.is_empty() {
                let mut successful_sends = 0;
                let mut failed_sends = 0;

                for member in &members {
                    let body = EmailKind::get_styled(
                        &EmailKind::Birthday,
                        None,
                        Some(member.clone()),
                        None,
                    );
                    let body = match body {
                        Ok(body_content) => body_content,
                        Err(err) => {
                            leptos::logging::log!(
                                "Failed to generate email body for member {}: {}",
                                member.id.unwrap_or(0),
                                err
                            );
                            failed_sends += 1;
                            continue; // skip this member
                        }
                    };

                    match member.id {
                        Some(member_id) => {
                            match Email::send_single(
                                &pool,
                                &email_client,
                                member_id,
                                String::from("Feliz Cumpleaños!"),
                                body,
                                String::from("Email de Cumpleaños por defecto."),
                            )
                            .await
                            {
                                Ok(()) => {
                                    successful_sends += 1;
                                }
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
                        }
                        None => {
                            failed_sends += 1;
                        }
                    }
                }

                if failed_sends > 0 && successful_sends == 0 {
                    leptos::logging::log!(
                        "COMPLETE FAILURE: Failed to send emails to any of the {} members",
                        members.len()
                    );
                    Err(ServerFnError::new(format!(
                        "Failed to send emails to any of the {} members",
                        members.len()
                    )))
                } else if failed_sends > 0 {
                    leptos::logging::log!(
                        "PARTIAL SUCCESS: sent {}/{} emails successfully, {} failed",
                        successful_sends,
                        &members.len(),
                        failed_sends
                    );
                    Err(ServerFnError::new(format!(
                        "Partially successful: sent {}/{} emails successfully, {} failed",
                        successful_sends,
                        members.len(),
                        failed_sends
                    )))
                } else {
                    leptos::logging::log!(
                        "COMPLETE SUCCESS: Successfully sent birthday emails to all {} members",
                        successful_sends
                    );
                    Ok(())
                }
            } else {
                leptos::logging::log!("No members have birthdays today ({})", today);
                Ok(())
            }
        }
        Err(e) => {
            leptos::logging::log!(
                "DATABASE ERROR: Failed to get today's birthday members: {}",
                e
            );
            Err(ServerFnError::new("Failed to retrieve birthday members"))
        }
    }
}

#[server(SendInterestsCuponEmail, "/api/emails/send-interests-cupon")]
pub async fn send_interests_cupon_email(
    interests: Vec<Interest>,
    cupon_description: String,
    cupon_expires_at: Option<chrono::NaiveDate>,
    subject: String,
    original_body: String,
) -> Result<(), ServerFnError> {
    use crate::core::email_client::EmailClient;
    use crate::core::models::cupon::Cupon;
    use crate::core::utils::cupon::generate_random_code;
    use crate::core::utils::email::EmailKind;

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
    let cupon_code = generate_random_code();

    for member in &members {
        match member.id {
            Some(member_id) => {
                let cupon = Cupon {
                    member_id: Some(member_id),
                    code: cupon_code.clone(),
                    description: cupon_description.clone(),
                    expires_at: cupon_expires_at,
                    ..Default::default()
                };
                let cupon_result = Cupon::add(&pool, cupon.clone()).await;
                if let Err(err) = cupon_result {
                    failed_sends += 1;
                    leptos::logging::log!("Error creating cupon: {}", err);
                    continue;
                };

                let body = EmailKind::get_styled(
                    &EmailKind::Cupon,
                    Some(original_body.clone()),
                    Some(member.clone()),
                    Some(cupon),
                );
                let body = if let Err(err) = body {
                    failed_sends += 1;
                    leptos::logging::log!("Error creating email template: {}", err);
                    continue;
                } else {
                    body.unwrap()
                };

                match Email::send_single(
                    &pool,
                    &email_client,
                    member_id,
                    subject.clone(),
                    body.clone(),
                    original_body.clone(),
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
