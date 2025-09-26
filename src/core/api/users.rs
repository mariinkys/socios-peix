// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

#[cfg(feature = "ssr")]
use actix_session::Session;
#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};
#[cfg(feature = "ssr")]
use std::sync::Arc;

use crate::core::models::user::User;

#[server(GetSessionUser)]
pub async fn get_user_from_session() -> Result<Option<User>, ServerFnError> {
    let ext_session: Option<Session> = extract().await?;

    if let Some(session) = ext_session {
        if let Some(user) = session.get::<User>("user")? {
            //leptos::logging::log!("SESSION value: {}", user);
            // TODO: Validate user in db? (not gonna do it for this project since it's self-hosted and does not need maximun security but I guess it would be alright?)
            // altho I'm not sure since this runs on every page reload/login/logout... keep in mind this does not mean when navigating through the webapp
            // it means when RELOADING the app
            return Ok(Some(user));
        } else {
            return Err(ServerFnError::new("Failed to retrieve user in session"));
        }
    } else {
        return Err(ServerFnError::new("Failed to retrieve session"));
    }
}

#[server(Login, "/api/user/login")]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    let ext_session: Option<Session> = extract().await?;
    let ext: Data<Pool<Sqlite>> = extract().await?;
    let pool: Arc<Pool<Sqlite>> = ext.into_inner();

    if let Some(session) = ext_session {
        let result = User::login(&pool, username, password).await;

        match result {
            Ok(user) => {
                session.insert("user", user)?;
                Ok(())
            }
            Err(e) => match e {
                crate::core::models::user::UserError::OperationFailed(err) => {
                    Err(ServerFnError::new(format!("Login failed: {}", err)))
                }
                crate::core::models::user::UserError::DatabaseError(error) => Err(
                    ServerFnError::new(format!("Login failed with database error: {}", error,)),
                ),
            },
        }
    } else {
        return Err(ServerFnError::new("Failed to retrieve session"));
    }
}

#[server(Logout, "/api/user/logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    let ext_session: Option<Session> = extract().await?;

    if let Some(session) = ext_session {
        session.clear();
        return Ok(());
    } else {
        return Err(ServerFnError::new("Failed to retrieve session"));
    }
}
