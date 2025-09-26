use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::{
    WildcardSegment,
    components::{Redirect, Route, Router, Routes},
    path,
};

use crate::{
    components::{navbar::NavbarComponent, toast::ToastComponent},
    core::api::users::{Login, Logout, get_user_from_session},
    pages::{
        cupons::cupon_check::CuponCheckPage,
        home::HomePage,
        interests::{list::InterestsPage, upsert::UpsertInterestPage},
        login::LoginPage,
        management::ManagementPage,
        members::{list::MembersPage, upsert::UpsertMemberPage},
        users::{list::UsersPage, upsert::UpsertUserPage},
    },
};

#[cfg(target_arch = "wasm32")]
use leptos::web_sys::window;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();

    let user_data = Resource::new(
        move || {
            (
                // changing these conditions may reduce "get_user_data" server calls
                login.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user_from_session(),
    );

    let dark_mode = RwSignal::new(false);

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        #[allow(clippy::collapsible_if)]
        if let Some(stored_theme) = get_stored_theme() {
            dark_mode.set(stored_theme);
        } else {
            if let Some(win) = window() {
                if let Ok(media_query) = win.match_media("(prefers-color-scheme: dark)") {
                    #[allow(clippy::collapsible_match)]
                    if let Some(mq) = media_query {
                        dark_mode.set(mq.matches());
                    }
                }
            }
        };
    });

    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        let theme = if dark_mode.get() { "dark" } else { "light" };

        #[cfg(target_arch = "wasm32")]
        {
            // apply theme
            if let Some(document) = web_sys::window().unwrap().document()
                && let Some(html) = document.document_element()
            {
                html.set_attribute("data-theme", theme).unwrap();
            }

            // save to localStorage
            save_theme_preference(dark_mode.get());
        }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/socios-peix.css"/>

        // sets the document title
        <Title text="Socios Peix"/>

        <ToastComponent/>
        <Transition>
            <Show
                when=move || is_logged_in(user_data.get())
                fallback=move || ()
            >
                <NavbarComponent dark_mode=dark_mode/>
            </Show>
        </Transition>

        // content for this welcome page
        <Router>
            <main class="p-3">
                <Routes fallback=move || "Not found.">
                    // PROTECTED ROUTES
                    <Route path=path!("") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <HomePage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=path!("/members") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <MembersPage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=path!("/members/:id") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <UpsertMemberPage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=path!("/interests") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <InterestsPage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=path!("/interests/:id") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <UpsertInterestPage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=path!("/management") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <ManagementPage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=path!("/users") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <UsersPage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=path!("/users/:id") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <UpsertUserPage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=path!("/cupon-check") view=move || view! {
                        <Transition>
                            <Show
                                when=move || is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/login" /> }
                            >
                                <CuponCheckPage/>
                            </Show>
                        </Transition>
                    }/>
                    // END OF PROTECTED ROUTES

                    <Route path=path!("/login") view=move || view! {
                        <Transition>
                            <Show
                                when=move || !is_logged_in(user_data.get())
                                fallback=move || view! { <Redirect path="/" /> }
                            >
                                <LoginPage/>
                            </Show>
                        </Transition>
                    }/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

fn is_logged_in(
    user_data: Option<Result<Option<crate::core::models::user::User>, ServerFnError>>,
) -> bool {
    match user_data {
        None => false,
        Some(Err(_)) => false,
        Some(Ok(None)) => false,
        Some(Ok(Some(_))) => true,
    }
}

#[cfg(target_arch = "wasm32")]
fn get_local_storage() -> Option<web_sys::Storage> {
    window()?.local_storage().ok().flatten()
}

// Get stored theme preference
#[cfg(target_arch = "wasm32")]
fn get_stored_theme() -> Option<bool> {
    let storage = get_local_storage()?;
    let theme = storage.get_item("theme").ok().flatten()?;
    match theme.as_str() {
        "dark" => Some(true),
        "light" => Some(false),
        _ => None,
    }
}

// Save theme preference
#[cfg(target_arch = "wasm32")]
fn save_theme_preference(is_dark: bool) {
    if let Some(storage) = get_local_storage() {
        let theme = if is_dark { "dark" } else { "light" };
        let _ = storage.set_item("theme", theme);
    }
}
