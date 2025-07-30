use leptos::prelude::*;

use crate::components::{
    emails::category_emails::CategoryEmails, management::refresh_cache::RefreshCacheComponent,
};

#[component]
pub fn ManagementPage() -> impl IntoView {
    view! {
        <div class="py-4 px-4 xl:px-96 flex flex-col gap-2">
            <h1 class="text-4xl font-bold">"Gestión"</h1>

            <div>
                <h2 class="text-2xl font-bold">"Emails"</h2>
                <div class="flex flex-wrap gap-2">
                    <CategoryEmails/>
                </div>
            </div>
            <div>
                <h2 class="text-2xl font-bold">"Otros"</h2>
                <div class="flex flex-wrap gap-2">
                    <RefreshCacheComponent/>
                </div>
            </div>
            <br/>
            <p class="text-center">"Made by Alex Marín - FLOSS, now and always! - "<a class="link-primary" href="https://github.com/mariinkys/" target="_blank">"v"{env!("CARGO_PKG_VERSION")}</a></p>
        </div>
    }
}
