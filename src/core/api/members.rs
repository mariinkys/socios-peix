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

use crate::core::models::member::{Member, MemberWithInterests};

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

#[server(AllMembers, "/api/members")]
pub async fn get_all_members() -> Result<Vec<Member>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Member::get_all(&pool).await;

    match result {
        Ok(members) => Ok(members),
        Err(e) => {
            leptos::logging::log!("Failed to get all members members: {}", e);
            Err(ServerFnError::new("Failed to retrieve all members"))
        }
    }
}

#[server(AllMembersWithInterests, "/api/members-with-interests")]
pub async fn get_all_members_with_interests() -> Result<Vec<MemberWithInterests>, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Member::get_all_with_interests(&pool).await;

    match result {
        Ok(members) => Ok(members),
        Err(e) => {
            leptos::logging::log!("Failed to get all members members: {}", e);
            Err(ServerFnError::new("Failed to retrieve all members"))
        }
    }
}

#[server(SingleMember, "/api/member")]
pub async fn get_member(member_id: i32) -> Result<Member, ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Member::get_single(&pool, member_id).await;

    match result {
        Ok(members) => Ok(members),
        Err(e) => {
            leptos::logging::log!("Failed to get single member: {}", e);
            Err(ServerFnError::new("Failed to retrieve single member"))
        }
    }
}

#[server(UpsertMember, "/api/member/upsert")]
pub async fn upsert_memeber(member: Member) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = match member.id {
        Some(_) => Member::edit(&pool, member).await,
        None => Member::add(&pool, member).await,
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            leptos::logging::log!("Failed to upsert member: {}", e);
            Err(ServerFnError::new("Failed to upsert member"))
        }
    }
}

#[server(DeleteMember, "/api/member/delete")]
pub async fn delete_memeber(member_id: i32) -> Result<(), ServerFnError> {
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    let result = Member::delete(&pool, member_id).await;

    match result {
        Ok(()) => Ok(()),
        Err(e) => {
            leptos::logging::log!("Failed to delete member: {}", e);
            Err(ServerFnError::new("Failed to delete member"))
        }
    }
}
