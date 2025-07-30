use leptos::prelude::*;

use crate::{components::page_loading::PageLoadingComponent, core::api::email::get_today_emails};

#[component]
pub fn TodayEmails() -> impl IntoView {
    let today_emails = Resource::new(|| (), |_| get_today_emails());

    let current_page = RwSignal::new(1usize);
    let items_per_page = RwSignal::new(8usize);
    let paginated_emails = move || {
        if let Some(Ok(today_emails)) = today_emails.get() {
            let page = current_page.get();
            let per_page = items_per_page.get();
            let start_idx = (page - 1) * per_page;

            Some(
                today_emails
                    .into_iter()
                    .skip(start_idx)
                    .take(per_page)
                    .collect::<Vec<_>>(),
            )
        } else {
            None
        }
    };
    let pagination_info = move || {
        if let Some(Ok(today_emails)) = today_emails.get() {
            let total_items = today_emails.len();
            let per_page = items_per_page.get();
            let total_pages = if total_items == 0 {
                1
            } else {
                total_items.div_ceil(per_page)
            };
            let current = current_page.get();

            (total_items, total_pages, current, per_page)
        } else {
            (0, 1, 1, items_per_page.get())
        }
    };

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex flex-col md:flex-row gap-3 md:gap-1">
                            <h2 class="card-title grow">"Emails enviados hoy"</h2>

                            <div class="flex flex-col md:flex-row gap-2 w-full md:w-auto">
                                <button class="btn btn-warning"
                                    on:click=move |_| today_emails.refetch()
                                >
                                    "Recargar"
                                </button>
                            </div>
                        </div>

                        <Show
                            when=move || { paginated_emails().is_some_and(|x| !x.is_empty()) }
                            fallback=|| view! { <p class="text-center mt-3">"No hay emails..."</p> }
                        >
                            <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200">
                                <table class="table">
                                    <thead>
                                    <tr>
                                        <th></th>
                                        <th>"Destinatario"</th>
                                        <th>"Asunto"</th>
                                        <th>"Contenido"</th>
                                        <th>"Enviado el Día"</th>
                                        <th class="text-right">"Ver"</th>
                                    </tr>
                                    </thead>
                                    <tbody>
                                        <For each=move || paginated_emails().unwrap_or_default() key=|m| m.id children=move |m| {
                                            view! {
                                                <tr>
                                                    <th>{m.id}</th>
                                                    <td>{m.member_full_name}</td>
                                                    <td>{m.subject}</td>
                                                    <td>{m.body}</td>
                                                    <td>{m.created_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                                    <td class="text-right"><a class="btn btn-primary" href=format!("/members/{}", m.member_id)>"Ver"</a></td>
                                                </tr>
                                            }
                                        }/>
                                    </tbody>
                                </table>
                            </div>

                            <div class="flex justify-center mt-4">
                                <div class="join">
                                    {move || {
                                        let (_, total_pages, current, _) = pagination_info();

                                        let prev_disabled = current <= 1;
                                        let prev_button = view! {
                                            <button class="join-item btn"
                                                class:btn-disabled=prev_disabled
                                                on:click=move |_| {
                                                    if current > 1 {
                                                        current_page.set(current - 1);
                                                    }
                                                }
                                            >"«"</button>
                                        };

                                        // Page numbers
                                        let mut page_buttons = Vec::new();
                                        let start_page = if current <= 3 { 1 } else { current - 2 };
                                        let end_page = std::cmp::min(start_page + 4, total_pages);
                                        let actual_start = if end_page - start_page < 4 && end_page >= 5 {
                                            std::cmp::max(1, end_page - 4)
                                        } else {
                                            start_page
                                        };

                                        for page in actual_start..=end_page {
                                            let is_current = page == current;
                                            page_buttons.push(view! {
                                                <button class="join-item btn"
                                                    class:btn-active=is_current
                                                    on:click=move |_| {
                                                        current_page.set(page);
                                                    }
                                                >{page}</button>
                                            });
                                        }

                                        let next_disabled = current >= total_pages;
                                        let next_button = view! {
                                            <button class="join-item btn"
                                                class:btn-disabled=next_disabled
                                                on:click=move |_| {
                                                    if current < total_pages {
                                                        current_page.set(current + 1);
                                                    }
                                                }
                                            >"»"</button>
                                        };

                                        view! {
                                            <div class="flex gap-1">
                                                {prev_button}
                                                <div>
                                                    {page_buttons.into_iter().collect_view()}
                                                </div>
                                                {next_button}
                                            </div>
                                        }
                                    }}
                                </div>
                            </div>
                        </Show>

                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
