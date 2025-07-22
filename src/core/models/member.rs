// SPDX-License-Identifier: GPL-3.0-only

use crate::core::entities::{country::Country, gender::Gender};
#[cfg(feature = "ssr")]
use crate::core::models::interest::Interest;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{Pool, Row, Sqlite};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Member {
    pub id: Option<i32>,
    pub name: String,
    pub surname: String,
    pub second_surname: String,
    pub email: String,
    pub birthdate: Option<NaiveDate>,
    pub phone: String,
    pub country: Country, // Country is a local_model, not a database table.
    pub gender: Gender,   // Gender us a local_model, not a database table.
    pub notes: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl std::fmt::Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.id.is_some_and(|x| x == other.id.unwrap_or_default())
    }
}

impl Member {
    /// Returns true if the entity is valid (ready for submission to the db)
    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() || self.surname.is_empty() {
            return false;
        }

        true
    }
}

#[cfg(feature = "ssr")]
impl Member {
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Member>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT 
                id, 
                name,
                surname,
                second_surname,
                email,
                birthdate,
                phone,
                country_id,
                gender_id,
                notes,
                created_at, 
                updated_at
            FROM members 
            ORDER BY id ASC",
        )
        .fetch_all(pool)
        .await?;

        let mut result = Vec::<Member>::new();

        for row in rows {
            let id: Option<i32> = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let surname: String = row.try_get("surname")?;
            let second_surname: String = row.try_get("second_surname")?;
            let email: String = row.try_get("email")?;
            let birthdate: Option<NaiveDate> = row.try_get("birthdate")?;
            let phone: String = row.try_get("phone")?;
            let country_id: i32 = row.try_get("country_id")?;
            let gender_id: i32 = row.try_get("gender_id")?;
            let notes: String = row.try_get("notes")?;
            let created_at: Option<NaiveDateTime> = row.try_get("created_at")?;
            let updated_at: Option<NaiveDateTime> = row.try_get("updated_at")?;

            let country = Country::from_id(country_id).unwrap_or_default();
            let gender = Gender::from_id(gender_id).unwrap_or_default();

            let member = Member {
                id,
                name,
                surname,
                second_surname,
                email,
                birthdate,
                phone,
                country,
                gender,
                notes,
                created_at,
                updated_at,
            };
            result.push(member);
        }
        Ok(result)
    }

    pub async fn get_all_with_interests(
        pool: &Pool<Sqlite>,
    ) -> Result<Vec<(Member, Vec<Interest>)>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT 
                id, 
                name,
                surname,
                second_surname,
                email,
                birthdate,
                phone,
                country_id,
                gender_id,
                notes,
                created_at, 
                updated_at
            FROM members 
            ORDER BY id ASC",
        )
        .fetch_all(pool)
        .await?;

        let mut result = Vec::<(Member, Vec<Interest>)>::new();

        for row in rows {
            let id: Option<i32> = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let surname: String = row.try_get("surname")?;
            let second_surname: String = row.try_get("second_surname")?;
            let email: String = row.try_get("email")?;
            let birthdate: Option<NaiveDate> = row.try_get("birthdate")?;
            let phone: String = row.try_get("phone")?;
            let country_id: i32 = row.try_get("country_id")?;
            let gender_id: i32 = row.try_get("gender_id")?;
            let notes: String = row.try_get("notes")?;
            let created_at: Option<NaiveDateTime> = row.try_get("created_at")?;
            let updated_at: Option<NaiveDateTime> = row.try_get("updated_at")?;

            let country = Country::from_id(country_id).unwrap_or_default();
            let gender = Gender::from_id(gender_id).unwrap_or_default();

            // Get interests for this member
            let interests = Interest::get_member_interests(pool, id.unwrap_or(0)).await?;

            let member = Member {
                id,
                name,
                surname,
                second_surname,
                email,
                birthdate,
                phone,
                country,
                gender,
                notes,
                created_at,
                updated_at,
            };
            result.push((member, interests));
        }
        Ok(result)
    }

    pub async fn get_single(pool: &Pool<Sqlite>, member_id: i32) -> Result<Member, sqlx::Error> {
        let row = sqlx::query(
            "SELECT 
                id, 
                name,
                surname,
                second_surname,
                email,
                birthdate,
                phone,
                country_id,
                gender_id,
                notes,
                created_at, 
                updated_at
            FROM members 
            WHERE id = $1",
        )
        .bind(member_id)
        .fetch_one(pool)
        .await?;

        let id: Option<i32> = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let surname: String = row.try_get("surname")?;
        let second_surname: String = row.try_get("second_surname")?;
        let email: String = row.try_get("email")?;
        let birthdate: Option<NaiveDate> = row.try_get("birthdate")?;
        let phone: String = row.try_get("phone")?;
        let country_id: i32 = row.try_get("country_id")?;
        let gender_id: i32 = row.try_get("gender_id")?;
        let notes: String = row.try_get("notes")?;
        let created_at: Option<NaiveDateTime> = row.try_get("created_at")?;
        let updated_at: Option<NaiveDateTime> = row.try_get("updated_at")?;

        let country = Country::from_id(country_id).unwrap_or_default();
        let gender = Gender::from_id(gender_id).unwrap_or_default();

        let member = Member {
            id,
            name,
            surname,
            second_surname,
            email,
            birthdate,
            phone,
            country,
            gender,
            notes,
            created_at,
            updated_at,
        };

        Ok(member)
    }

    pub async fn add(pool: &Pool<Sqlite>, member: Member) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO members (name, surname, second_surname, email, birthdate, phone, country_id, gender_id, notes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(member.name)
            .bind(member.surname)
            .bind(member.second_surname)
            .bind(member.email)
            .bind(member.birthdate)
            .bind(member.phone)
            .bind(member.country.to_id())
            .bind(member.gender.to_id())
            .bind(member.notes)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn edit(pool: &Pool<Sqlite>, member: Member) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE members SET name = $1, surname = $2, second_surname = $3, birthdate = $4, phone = $5, country_id = $6, gender_id = $7, notes = $8, email = $9, updated_at = CURRENT_TIMESTAMP WHERE id = $10")
            .bind(member.name)
            .bind(member.surname)
            .bind(member.second_surname)
            .bind(member.birthdate)
            .bind(member.phone)
            .bind(member.country.to_id())
            .bind(member.gender.to_id())
            .bind(member.notes)
            .bind(member.email)
            .bind(member.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM members WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    // Gets all members whose birthday is today (same day and month, regardless of year)
    pub async fn get_today_members(
        pool: &Pool<Sqlite>,
        today: NaiveDate,
    ) -> Result<Vec<Member>, sqlx::Error> {
        use chrono::Datelike;

        let today_day = today.day();
        let today_month = today.month();

        let rows = sqlx::query(
            "SELECT 
                id, 
                name,
                surname,
                second_surname,
                email,
                birthdate,
                phone,
                country_id,
                gender_id,
                notes,
                created_at, 
                updated_at
            FROM members 
            WHERE birthdate IS NOT NULL
                AND CAST(strftime('%d', birthdate) AS INTEGER) = $1
                AND CAST(strftime('%m', birthdate) AS INTEGER) = $2
            ORDER BY name ASC",
        )
        .bind(today_day as i32)
        .bind(today_month as i32)
        .fetch_all(pool)
        .await?;

        let mut result = Vec::<Member>::new();

        for row in rows {
            let id: Option<i32> = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let surname: String = row.try_get("surname")?;
            let second_surname: String = row.try_get("second_surname")?;
            let email: String = row.try_get("email")?;
            let birthdate: Option<NaiveDate> = row.try_get("birthdate")?;
            let phone: String = row.try_get("phone")?;
            let country_id: i32 = row.try_get("country_id")?;
            let gender_id: i32 = row.try_get("gender_id")?;
            let notes: String = row.try_get("notes")?;
            let created_at: Option<NaiveDateTime> = row.try_get("created_at")?;
            let updated_at: Option<NaiveDateTime> = row.try_get("updated_at")?;

            let country = Country::from_id(country_id).unwrap_or_default();
            let gender = Gender::from_id(gender_id).unwrap_or_default();

            let member = Member {
                id,
                name,
                surname,
                second_surname,
                email,
                birthdate,
                phone,
                country,
                gender,
                notes,
                created_at,
                updated_at,
            };
            result.push(member);
        }
        Ok(result)
    }
}
