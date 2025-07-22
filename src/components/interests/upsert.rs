use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::{
    components::{
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{api::interests::UpsertInterest, models::interest::Interest},
};

#[component]
pub fn UpsertInterest(edit_mode: RwSignal<bool>, model: RwSignal<Interest>) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let upsert_interest_action = ServerAction::<UpsertInterest>::new();
    let upsert_value = upsert_interest_action.value();
    Effect::new(move |_| {
        if let Some(val) = upsert_value.get() {
            let navigate = use_navigate();

            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Guardado"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    navigate("/interests", Default::default());
                }
                Err(err) => {
                    set_toast.set(ToastMessage {
                        message: format!("Error: {err}"),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                }
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // Stop the page from reloading
        ev.prevent_default();
        let interest_model = model.get();

        if interest_model.is_valid() {
            upsert_interest_action.dispatch(UpsertInterest {
                interest: interest_model,
            });
        } else {
            set_toast.set(ToastMessage {
                message: String::from("Faltan campos obligatorios"),
                toast_type: ToastType::Error,
                visible: true,
            });
        }
    };

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
            <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex gap-1">
                            <h2 class="card-title grow">"Interés"</h2>
                        </div>
                        <form class="w-full" on:submit=on_submit>
                            <div class="flex flex-col gap-2 w-full">
                                // name
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="name">"Nombre*"</label>
                                        <input type="text"
                                            class="input w-full"
                                            name="name"
                                            id="name"
                                            required
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value={move || model.get().name}
                                            on:input=move |ev| {
                                                model.update(|curr| {
                                                    curr.name = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                // description
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="description">"Descripción"</label>
                                        <input type="text"
                                            class="input w-full"
                                            name="description"
                                            id="description"
                                            required
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value={move || model.get().description}
                                            on:input=move |ev| {
                                                model.update(|curr| {
                                                    curr.description = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                <button type="submit" class="btn btn-primary w-full" disabled={ move || !edit_mode.get() } >"Guardar"</button>

                            </div>
                        </form>
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
