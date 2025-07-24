// SPDX-License-Identifier: GPL-3.0-only

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

#[cfg(feature = "ssr")]
use crate::core::email_client::EmailClient;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Email {
    pub id: Option<i32>,
    pub to_member_id: i32,
    pub subject: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
}

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        match (self.id, other.id) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }
}

impl Email {
    /// Returns true if the entity is valid (ready for submission to the db)
    pub fn is_valid(&self) -> bool {
        if self.subject.is_empty() || self.body.is_empty() {
            return false;
        }

        true
    }
}

#[cfg(feature = "ssr")]
impl Email {
    /// Get all emails sent to a specific member
    pub async fn get_by_member(
        pool: &Pool<Sqlite>,
        member_id: i32,
    ) -> Result<Vec<Email>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, to_member_id, subject, body, created_at
             FROM emails
             WHERE to_member_id = $1
             ORDER BY created_at DESC",
        )
        .bind(member_id)
        .fetch_all(pool)
        .await?;

        let mut emails = Vec::new();
        for row in rows {
            emails.push(Email {
                id: row.try_get("id")?,
                to_member_id: row.try_get("to_member_id")?,
                subject: row.try_get("subject")?,
                body: row.try_get("body")?,
                created_at: row.try_get("created_at")?,
            });
        }

        Ok(emails)
    }

    /// Get all emails
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Email>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT id, to_member_id, subject, body, created_at
             FROM emails
             ORDER BY created_at DESC",
        )
        .fetch_all(pool)
        .await?;

        let mut emails = Vec::new();
        for row in rows {
            emails.push(Email {
                id: row.try_get("id")?,
                to_member_id: row.try_get("to_member_id")?,
                subject: row.try_get("subject")?,
                body: row.try_get("body")?,
                created_at: row.try_get("created_at")?,
            });
        }

        Ok(emails)
    }

    /// Get a single email by ID
    pub async fn get_single(pool: &Pool<Sqlite>, email_id: i32) -> Result<Email, sqlx::Error> {
        let row = sqlx::query(
            "SELECT id, to_member_id, subject, body, created_at
             FROM emails
             WHERE id = $1",
        )
        .bind(email_id)
        .fetch_one(pool)
        .await?;

        Ok(Email {
            id: row.try_get("id")?,
            to_member_id: row.try_get("to_member_id")?,
            subject: row.try_get("subject")?,
            body: row.try_get("body")?,
            created_at: row.try_get("created_at")?,
        })
    }

    /// Add a new email
    pub async fn add(pool: &Pool<Sqlite>, email: Email) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO emails (to_member_id, subject, body, created_at)
             VALUES ($1, $2, $3, CURRENT_TIMESTAMP)",
        )
        .bind(email.to_member_id)
        .bind(email.subject)
        .bind(email.body)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Delete an email by ID
    pub async fn delete(pool: &Pool<Sqlite>, email_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM emails WHERE id = $1")
            .bind(email_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Attempts to send an email to a specific member
    pub async fn send_single(
        pool: &Pool<Sqlite>,
        email_client: &EmailClient,
        member_id: i32,
        subject: String,
        body: String,
    ) -> Result<(), anyhow::Error> {
        use crate::core::models::member::Member;
        use anyhow::{anyhow, Context};
        use lettre::message::{header::ContentType, Mailbox};
        use lettre::Transport;

        // Get member with proper error handling
        let member = Member::get_single(pool, member_id)
            .await
            .with_context(|| format!("Failed to fetch member with ID: {member_id}"))?;

        // Validate email address is not empty
        if member.email.is_empty() {
            return Err(anyhow!("Member {} has no email address", member_id));
        }

        // Parse email addresses with proper error handling
        let from_email = email_client
            .from_email
            .parse()
            .with_context(|| format!("Invalid from email address: {}", email_client.from_email))?;

        let to_email = member
            .email
            .parse()
            .with_context(|| format!("Invalid member email address: {}", member.email))?;

        // Build email message
        let email = lettre::Message::builder()
            .from(Mailbox::new(
                Some(email_client.from_name.clone()),
                from_email,
            ))
            .to(Mailbox::new(Some(member.to_string()), to_email))
            .subject(&subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.clone())
            .with_context(|| "Failed to build email message")?;

        // Send email
        email_client
            .mailer
            .send(&email)
            .with_context(|| format!("Failed to send email to member {member_id}"))?;

        // Record email in database
        Email::add(
            pool,
            Email {
                id: None,
                to_member_id: member_id,
                subject,
                body,
                created_at: None,
            },
        )
        .await
        .with_context(|| "Failed to record email in database")?;

        Ok(())
    }
}
