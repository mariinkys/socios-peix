// SPDX-License-Identifier: GPL-3.0-only

#[cfg(feature = "ssr")]
use crate::core::utils::passwords::verify_password;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id.is_some_and(|x| x == other.id.unwrap_or_default())
    }
}

#[cfg(feature = "ssr")]
pub enum UserError {
    OperationFailed(String),
    DatabaseError(sqlx::Error),
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for UserError {
    fn from(value: sqlx::Error) -> Self {
        let sqlx::Error::RowNotFound = &value else {
            return UserError::DatabaseError(value);
        };
        UserError::OperationFailed(String::from("Username not found"))
    }
}

impl User {
    #[cfg(feature = "ssr")]
    pub async fn login(
        pool: &Pool<Sqlite>,
        username: String,
        password: String,
    ) -> Result<User, UserError> {
        let row = sqlx::query(
            "SELECT 
                id, 
                username,
                password,
                created_at, 
                updated_at
            FROM users 
            WHERE username = $1",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        let id: Option<i32> = row.try_get("id")?;
        let username: String = row.try_get("username")?;
        let hash: String = row.try_get("password")?;
        let created_at: Option<NaiveDateTime> = row.try_get("created_at")?;
        let updated_at: Option<NaiveDateTime> = row.try_get("updated_at")?;

        let is_password_valid = verify_password(password, hash);
        if let Err(password_error) = is_password_valid {
            return Err(UserError::OperationFailed(password_error));
        }

        let user = User {
            id,
            username,
            password: String::new(),
            created_at,
            updated_at,
        };

        Ok(user)
    }
}
