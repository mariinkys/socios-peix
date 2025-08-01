// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

use crate::{
    components::{page_loading::PageLoadingComponent, toast::{ToastMessage, ToastType}}, core::api::email::TestEmailConfig,
};

#[component]
pub fn TestEmailConfig() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let test_config = ServerAction::<TestEmailConfig>::new();

    let value = test_config.value();
    Effect::new(move |_| {
        if let Some(val) = value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Connection is Ok!"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                }
                Err(err) => {
                    set_toast.set(ToastMessage {
                        message: format!("{err}"),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                }
            }
        }
    });
    
    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="card bg-base-100 shadow-xl w-80">
                    <ActionForm action=test_config attr:class="h-full">
                        <div class="card-body w-full h-full justify-between">
                            <p class="text-xl text-center">"Probar Conexión a Servidor Email"</p>
                            <button class="btn btn-success w-full"
                                on:click=move |_| {
                                
                            }>
                                "Probar"
                            </button>
                        </div>
                    </ActionForm>
                </div>
            </ErrorBoundary>
        </Suspense>
    }.into_any()
}