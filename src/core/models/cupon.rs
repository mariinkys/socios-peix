// SPDX-License-Identifier: GPL-3.0-only

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cupon {
    pub id: Option<i32>,
    pub member_id: Option<i32>,
    pub code: String,
    pub description: String,
    pub used: bool,
    pub is_deleted: bool,
    pub expires_at: Option<NaiveDate>,
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

    pub async fn get_cupons_by_code(
        pool: &Pool<Sqlite>,
        code: &str,
        today: NaiveDate,
    ) -> Result<Vec<Cupon>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT cupons.* 
         FROM cupons 
         WHERE cupons.code = $1
           AND (expires_at IS NULL OR DATE(expires_at) >= DATE($2))",
        )
        .bind(code)
        .bind(today)
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

    /// Add a new cupon
    pub async fn add(pool: &Pool<Sqlite>, cupon: Cupon) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO cupons (member_id, code, description, expires_at, created_at)
             VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)",
        )
        .bind(cupon.member_id)
        .bind(cupon.code)
        .bind(cupon.description)
        .bind(cupon.expires_at)
        .execute(pool)
        .await?;

        Ok(())
    }
}
