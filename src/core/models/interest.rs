// SPDX-License-Identifier: GPL-3.0-only

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[cfg(feature = "ssr")]
impl Interest {
    pub async fn get_member_interests(
        pool: &Pool<Sqlite>,
        member_id: i32,
    ) -> Result<Vec<Interest>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT interests.* FROM interests 
             INNER JOIN member_interests ON interests.id = member_interests.interest_id 
             WHERE member_interests.member_id = $1",
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
}
