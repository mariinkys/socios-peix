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

use crate::core::models::cupon::Cupon;

#[server(MemberInterests, "/api/emails/member")]
pub async fn get_member_cupons(member_id: i32) -> Result<Vec<Cupon>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Cupon::get_member_cupons(&pool, member_id).await;

    match result {
        Ok(cupons) => Ok(cupons),
        Err(e) => {
            leptos::logging::log!("Failed to get all emails: {}", e);
            Err(ServerFnError::new("Failed to retrieve all emails"))
        }
    }
}
