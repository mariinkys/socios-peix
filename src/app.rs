use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::{
    WildcardSegment,
    components::{Route, Router, Routes},
    path,
};

use crate::{
    components::{navbar::NavbarComponent, toast::ToastComponent},
    pages::{
        cupons::cupon_check::CuponCheckPage,
        home::HomePage,
        interests::{list::InterestsPage, upsert::UpsertInterestPage},
        management::ManagementPage,
        members::{list::MembersPage, upsert::UpsertMemberPage},
    },
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/socios-peix.css"/>

        // sets the document title
        <Title text="Socios Peix"/>

        <ToastComponent/>
        <NavbarComponent/>

        // content for this welcome page
        <Router>
            <main class="p-3">
                <Routes fallback=move || "Not found.">
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/members") view=MembersPage/>
                    <Route path=path!("/members/:id") view=UpsertMemberPage/>
                    <Route path=path!("/interests") view=InterestsPage/>
                    <Route path=path!("/interests/:id") view=UpsertInterestPage/>
                    <Route path=path!("/management") view=ManagementPage/>
                    <Route path=path!("/cupon-check") view=CuponCheckPage/>
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
