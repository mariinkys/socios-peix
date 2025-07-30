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

use crate::core::models::interest::Interest;

#[server(AllInterests, "/api/interests")]
pub async fn get_all_interests() -> Result<Vec<Interest>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Interest::get_all(&pool).await;

    match result {
        Ok(interests) => Ok(interests),
        Err(e) => {
            leptos::logging::log!("Failed to get all interests: {}", e);
            Err(ServerFnError::new("Failed to retrieve all interests"))
        }
    }
}

#[server(SingleInterest, "/api/interest")]
pub async fn get_interest(interest_id: i32) -> Result<Interest, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Interest::get_single(&pool, interest_id).await;

    match result {
        Ok(interests) => Ok(interests),
        Err(e) => {
            leptos::logging::log!("Failed to get single interest: {}", e);
            Err(ServerFnError::new("Failed to retrieve single interest"))
        }
    }
}

#[server(UpsertInterest, "/api/interest/upsert")]
pub async fn upsert_memeber(interest: Interest) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = match interest.id {
        Some(_) => Interest::edit(&pool, interest).await,
        None => Interest::add(&pool, interest).await,
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            leptos::logging::log!("Failed to upsert interest: {}", e);
            Err(ServerFnError::new("Failed to upsert interest"))
        }
    }
}

#[server(MemberInterests, "/api/interests/member")]
pub async fn get_member_interests(member_id: i32) -> Result<Vec<Interest>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Interest::get_member_interests(&pool, member_id).await;

    match result {
        Ok(interests) => Ok(interests),
        Err(e) => {
            leptos::logging::log!("Failed to get all interests: {}", e);
            Err(ServerFnError::new("Failed to retrieve all interests"))
        }
    }
}

#[server(AddMemberInterest, "/api/interest/add/member")]
pub async fn add_memeber_interest(member_id: i32, interest_id: i32) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Interest::add_member_interest(&pool, member_id, interest_id).await;

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            leptos::logging::log!("Failed to add member interest: {}", e);
            Err(ServerFnError::new("Failed to add member interest"))
        }
    }
}

#[server(RemoveMemberInterest, "/api/interest/remove/member")]
pub async fn remove_memeber_interest(
    member_id: i32,
    interest_id: i32,
) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Interest::remove_member_interest(&pool, member_id, interest_id).await;

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            leptos::logging::log!("Failed to remove member interest: {}", e);
            Err(ServerFnError::new("Failed to remove member interest"))
        }
    }
}

#[server(DeleteInterest, "/api/interest/delete")]
pub async fn delete_interest(interest_id: i32) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Interest::delete(&pool, interest_id).await;

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            leptos::logging::log!("Failed to delete interest: {}", e);
            Err(ServerFnError::new("Failed to delete interest"))
        }
    }
}
