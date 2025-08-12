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
use crate::core::models::cupon::CuponWithMember;

#[server(MemberCupons, "/api/cupons/member")]
pub async fn get_member_cupons(member_id: i32) -> Result<Vec<Cupon>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Cupon::get_member_cupons(&pool, member_id).await;

    match result {
        Ok(cupons) => Ok(cupons),
        Err(e) => {
            leptos::logging::log!("Failed to get member cupons: {}", e);
            Err(ServerFnError::new("Failed to retrieve member cupons"))
        }
    }
}

#[server(CuponsByCode, "/api/cupons/code")]
pub async fn get_cupons_by_code(cupon: String) -> Result<Vec<CuponWithMember>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let today = chrono::Local::now().naive_local().date();
    let result = Cupon::get_cupons_by_code(&pool, cupon.trim(), today).await;

    match result {
        Ok(cupons) => Ok(cupons),
        Err(e) => {
            leptos::logging::log!("Failed to get cupons: {}", e);
            Err(ServerFnError::new("Failed to retrieve cupons"))
        }
    }
}

#[server(SwapCuponUsed, "/api/cupons/swap")]
pub async fn swap_cupon_used(cupon_id: i32, current_value: bool) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let new_value = !current_value;
    let result = Cupon::update_used(&pool, cupon_id, new_value).await;

    match result {
        Ok(cupons) => Ok(cupons),
        Err(e) => {
            leptos::logging::log!("Failed to get cupons: {}", e);
            Err(ServerFnError::new("Failed to retrieve cupons"))
        }
    }
}
