use leptos::prelude::*;

use crate::components::{
    emails::{
        category_cupon_emails::CategoryCuponEmails, category_emails::CategoryEmails,
        send_all_cupon_email::AllCuponEmails, send_all_email::AllEmails,
    },
    management::{refresh_cache::RefreshCacheComponent, test_email_connection::TestEmailConfig},
};

#[component]
pub fn ManagementPage() -> impl IntoView {
    view! {
        <div class="py-4 px-4 xl:px-64 flex flex-col gap-2">
            <h1 class="text-4xl font-bold">"Gestión"</h1>

            <div>
                <h2 class="text-2xl font-bold">"Emails"</h2>
                <div class="flex flex-wrap gap-2">
                    <TestEmailConfig/>
                    <CategoryEmails/>
                    <AllEmails/>
                </div>
            </div>
            <div>
                <h2 class="text-2xl font-bold">"Cupones"</h2>
                <div class="flex flex-wrap gap-2">
                    <CategoryCuponEmails/>
                    <AllCuponEmails/>
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
