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

use crate::core::models::member::Member;

#[server(GetTodayMembers, "/api/today-members")]
pub async fn get_today_members() -> Result<Vec<Member>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let today = chrono::Local::now().naive_local().date();
    let result = Member::get_today_members(&pool, today).await;

    match result {
        Ok(members) => Ok(members),
        Err(e) => {
            leptos::logging::log!("Failed to get today's birthday members: {}", e);
            Err(ServerFnError::new("Failed to retrieve birthday members"))
        }
    }
}
