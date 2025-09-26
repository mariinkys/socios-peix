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

#[cfg(feature = "ssr")]
impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
            UserError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserUpsertModel {
    pub id: Option<i32>,
    pub username: String,
    pub old_password: String,
    pub new_password: String,
    pub new_password_repeat: String,
}

#[derive(Debug, PartialEq)]
pub enum UpsertOperation {
    Add,
    Edit,
    PasswordChange,
}

impl UserUpsertModel {
    /// Returns true if the entity is valid (ready for submission to the db)
    pub fn is_valid(&self, operation: UpsertOperation) -> bool {
        match operation {
            UpsertOperation::Add => {
                if self.username.is_empty() {
                    return false;
                }
                if self.new_password.is_empty() || self.new_password_repeat.is_empty() {
                    return false;
                }
                if self.new_password != self.new_password_repeat {
                    return false;
                }
            }
            UpsertOperation::Edit => {
                if self.username.is_empty() {
                    return false;
                }
            }
            UpsertOperation::PasswordChange => {
                if self.new_password.is_empty() || self.new_password_repeat.is_empty() {
                    return false;
                }
                if self.new_password != self.new_password_repeat {
                    return false;
                }
                if self.old_password.is_empty() {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(feature = "ssr")]
impl User {
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

    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<User>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT 
                id, 
                username,
                created_at, 
                updated_at
            FROM users 
            ORDER BY id DESC",
        )
        .fetch_all(pool)
        .await?;

        let mut result = Vec::<User>::new();

        for row in rows {
            let id: Option<i32> = row.try_get("id")?;
            let username: String = row.try_get("username")?;
            let created_at: Option<NaiveDateTime> = row.try_get("created_at")?;
            let updated_at: Option<NaiveDateTime> = row.try_get("updated_at")?;

            let user = User {
                id,
                username,
                password: String::new(),
                created_at,
                updated_at,
            };
            result.push(user);
        }
        Ok(result)
    }

    pub async fn add(pool: &Pool<Sqlite>, user: UserUpsertModel) -> Result<(), UserError> {
        let hashed_password = crate::core::utils::passwords::encrypt_password(user.new_password);
        if let Err(err) = hashed_password {
            return Err(UserError::OperationFailed(err));
        };

        sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
            .bind(user.username)
            .bind(hashed_password.unwrap())
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn edit(pool: &Pool<Sqlite>, user: UserUpsertModel) -> Result<(), UserError> {
        sqlx::query("UPDATE users SET username = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
            .bind(user.username)
            .bind(user.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_single(
        pool: &Pool<Sqlite>,
        user_id: i32,
    ) -> Result<UserUpsertModel, sqlx::Error> {
        let row = sqlx::query(
            "SELECT 
                id, 
                username
            FROM users 
            WHERE id = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let id: Option<i32> = row.try_get("id")?;
        let username: String = row.try_get("username")?;

        let user = UserUpsertModel {
            id,
            username,
            ..Default::default()
        };

        Ok(user)
    }

    pub async fn change_password(
        pool: &Pool<Sqlite>,
        user_id: Option<i32>,
        old_password: String,
        new_password: String,
    ) -> Result<(), UserError> {
        let row = sqlx::query("SELECT password FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        let hash: String = row.try_get("password")?;

        let is_password_valid = verify_password(old_password, hash);
        if let Err(_err) = is_password_valid {
            return Err(UserError::OperationFailed(String::from(
                "Old password is incorrect",
            )));
        }

        let new_hashed_password = crate::core::utils::passwords::encrypt_password(new_password);
        if let Err(err) = new_hashed_password {
            return Err(UserError::OperationFailed(err));
        };

        sqlx::query("UPDATE users SET password = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
            .bind(new_hashed_password.unwrap())
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
