// SPDX-License-Identifier: GPL-3.0-only

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cupon {
    pub id: Option<i32>,
    pub member_id: Option<i32>,
    pub code: String,
    pub description: String,
    pub used: bool,
    pub is_deleted: bool,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl std::fmt::Display for Cupon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[cfg(feature = "ssr")]
impl Cupon {
    pub async fn get_member_cupons(
        pool: &Pool<Sqlite>,
        member_id: i32,
    ) -> Result<Vec<Cupon>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT cupons.* FROM cupons 
             WHERE cupons.member_id = $1",
        )
        .bind(member_id)
        .fetch_all(pool)
        .await?;

        let mut cupons = Vec::<Cupon>::new();
        for row in rows {
            let cupon = Cupon {
                id: row.try_get("id")?,
                member_id: row.try_get("member_id")?,
                code: row.try_get("code")?,
                used: row.try_get("used")?,
                description: row.try_get("description")?,
                expires_at: row.try_get("expires_at")?,
                is_deleted: row.try_get("is_deleted")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            };
            cupons.push(cupon);
        }

        Ok(cupons)
    }
}
