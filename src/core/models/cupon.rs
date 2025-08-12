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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CuponWithMember {
    pub cupon: Cupon,
    pub member: crate::core::models::member::Member,
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
             WHERE cupons.member_id = $1
             ORDER BY id DESC",
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

    pub async fn get_cupons_by_code(
        pool: &Pool<Sqlite>,
        code: &str,
        today: NaiveDate,
    ) -> Result<Vec<CuponWithMember>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT 
            cupons.id AS cupon_id,
            cupons.member_id AS cupon_member_id,
            cupons.code AS cupon_code,
            cupons.description AS cupon_description,
            cupons.used AS cupon_used,
            cupons.expires_at AS cupon_expires_at,
            cupons.is_deleted AS cupon_is_deleted,
            cupons.created_at AS cupon_created_at,
            cupons.updated_at AS cupon_updated_at,
            members.id AS member_id,
            members.name AS member_name,
            members.surname AS member_surname,
            members.second_surname AS member_second_surname,
            members.email AS member_email,
            members.birthdate AS member_birthdate,
            members.phone AS member_phone,
            members.country_id AS member_country_id,
            members.gender_id AS member_gender_id,
            members.notes AS member_notes,
            members.created_at AS member_created_at,
            members.updated_at AS member_updated_at
         FROM cupons
         LEFT JOIN members ON cupons.member_id = members.id
         WHERE cupons.code = $1
           AND (cupons.expires_at IS NULL OR DATE(cupons.expires_at) >= DATE($2))
         ORDER BY cupons.id DESC",
        )
        .bind(code)
        .bind(today)
        .fetch_all(pool)
        .await?;

        let mut cupons_with_members = Vec::<CuponWithMember>::new();

        for row in rows {
            let cupon = Cupon {
                id: row.try_get("cupon_id")?,
                member_id: row.try_get("cupon_member_id")?,
                code: row.try_get("cupon_code")?,
                used: row.try_get("cupon_used")?,
                description: row.try_get("cupon_description")?,
                expires_at: row.try_get("cupon_expires_at")?,
                is_deleted: row.try_get("cupon_is_deleted")?,
                created_at: row.try_get("cupon_created_at")?,
                updated_at: row.try_get("cupon_updated_at")?,
            };

            let member = crate::core::models::member::Member {
                id: row.try_get("member_id")?,
                name: row.try_get("member_name")?,
                surname: row.try_get("member_surname")?,
                second_surname: row.try_get("member_second_surname")?,
                email: row.try_get("member_email")?,
                phone: row.try_get("member_phone")?,
                ..Default::default()
            };

            cupons_with_members.push(CuponWithMember { cupon, member });
        }

        Ok(cupons_with_members)
    }
}
