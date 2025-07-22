// SPDX-License-Identifier: GPL-3.0-only

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Interest {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub is_deleted: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl std::fmt::Display for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Interest {
    /// Returns true if the entity is valid (ready for submission to the db)
    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() {
            return false;
        }

        true
    }
}

#[cfg(feature = "ssr")]
impl Interest {
    /// Returns all interests with their selection status for a specific member
    pub async fn get_member_interests(
        pool: &Pool<Sqlite>,
        member_id: i32,
    ) -> Result<Vec<Interest>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT 
                    interests.id,
                    interests.name,
                    interests.description,
                    interests.is_deleted,
                    interests.created_at,
                    interests.updated_at
                FROM interests
                INNER JOIN member_interests 
                    ON interests.id = member_interests.interest_id
                WHERE member_interests.member_id = $1
                AND interests.is_deleted = 0
                ORDER BY interests.name",
        )
        .bind(member_id)
        .fetch_all(pool)
        .await?;

        let mut interests = Vec::<Interest>::new();

        for row in rows {
            let interest = Interest {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                description: row.try_get("description").unwrap_or_default(),
                is_deleted: row.try_get("is_deleted")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            };

            interests.push(interest);
        }

        Ok(interests)
    }

    /// Adds the given interest to the given member
    pub async fn add_member_interest(
        pool: &Pool<Sqlite>,
        member_id: i32,
        interest_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO member_interests (member_id, interest_id, created_at) 
                VALUES ($1, $2, datetime('now'))",
        )
        .bind(member_id)
        .bind(interest_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Removes the given interest of the given member
    pub async fn remove_member_interest(
        pool: &Pool<Sqlite>,
        member_id: i32,
        interest_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM member_interests WHERE member_id = $1 AND interest_id = $2")
            .bind(member_id)
            .bind(interest_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Interest>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT 
            id, 
            name,
            description,
            is_deleted,
            created_at, 
            updated_at
        FROM interests 
        WHERE is_deleted = false
        ORDER BY id ASC",
        )
        .fetch_all(pool)
        .await?;

        let mut result = Vec::<Interest>::new();

        for row in rows {
            let id: Option<i32> = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let description: String = row.try_get("description")?;
            let is_deleted: bool = row.try_get("is_deleted")?;
            let created_at: Option<NaiveDateTime> = row.try_get("created_at")?;
            let updated_at: Option<NaiveDateTime> = row.try_get("updated_at")?;

            let interest = Interest {
                id,
                name,
                description,
                is_deleted,
                created_at,
                updated_at,
            };
            result.push(interest);
        }
        Ok(result)
    }

    pub async fn get_single(
        pool: &Pool<Sqlite>,
        interest_id: i32,
    ) -> Result<Interest, sqlx::Error> {
        let row = sqlx::query(
            "SELECT 
            id, 
            name,
            description,
            is_deleted,
            created_at, 
            updated_at
        FROM interests 
        WHERE id = $1 AND is_deleted = false",
        )
        .bind(interest_id)
        .fetch_one(pool)
        .await?;

        let id: Option<i32> = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let description: String = row.try_get("description")?;
        let is_deleted: bool = row.try_get("is_deleted")?;
        let created_at: Option<NaiveDateTime> = row.try_get("created_at")?;
        let updated_at: Option<NaiveDateTime> = row.try_get("updated_at")?;

        let interest = Interest {
            id,
            name,
            description,
            is_deleted,
            created_at,
            updated_at,
        };

        Ok(interest)
    }

    pub async fn add(pool: &Pool<Sqlite>, interest: Interest) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO interests (name, description, is_deleted) VALUES ($1, $2, $3)")
            .bind(interest.name)
            .bind(interest.description)
            .bind(interest.is_deleted)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn edit(pool: &Pool<Sqlite>, interest: Interest) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE interests SET name = $1, description = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND is_deleted = false")
        .bind(interest.name)
        .bind(interest.description)
        .bind(interest.id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE interests SET is_deleted = true, updated_at = CURRENT_TIMESTAMP WHERE id = $1",
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
