// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::{
    components::{
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::users::UpsertUser,
        models::user::{UpsertOperation, UserUpsertModel},
    },
};

#[component]
pub fn UpsertUser(edit_mode: RwSignal<bool>, model: RwSignal<UserUpsertModel>) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let upsert_user_action = ServerAction::<UpsertUser>::new();
    let upsert_value = upsert_user_action.value();
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
                    navigate("/users", Default::default());
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
        let user_model = model.get();

        let operation = if user_model.id.is_none() {
            UpsertOperation::Add
        } else {
            UpsertOperation::Edit
        };

        if user_model.is_valid(operation) {
            upsert_user_action.dispatch(UpsertUser { user: user_model });
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
                            <h2 class="card-title grow">"Usuario"</h2>
                        </div>
                        <form class="w-full" on:submit=on_submit>
                            <div class="flex flex-col gap-2 w-full">
                                // username
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="username">"Nombre de Usuario"</label>
                                        <input type="text"
                                            class="input w-full"
                                            name="username"
                                            id="username"
                                            required
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value={move || model.get().username}
                                            on:input=move |ev| {
                                                model.update(|curr| {
                                                    curr.username = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                // old password (if any)
                                // <div class="w-full">
                                //     <fieldset class="fieldset">
                                //         <label class="label" for="old_password">"Antigua Contraseña"</label>
                                //         <input type="password"
                                //             class="input w-full"
                                //             name="old_password"
                                //             id="old_password"
                                //             autocomplete="off"
                                //             disabled={move || !edit_mode.get() }
                                //             prop:value={move || model.get().old_password}
                                //             on:input=move |ev| {
                                //                 model.update(|curr| {
                                //                     curr.old_password = event_target_value(&ev);
                                //                 });
                                //             }
                                //         />
                                //     </fieldset>
                                // </div>

                                // we're creating a user
                                <Show
                                    when=move || model.get().id.is_none()
                                    fallback=move || ()
                                >
                                    // new password
                                    <div class="w-full">
                                        <fieldset class="fieldset">
                                            <label class="label" for="new_password">"Nueva Contraseña"</label>
                                            <input type="password"
                                                class="input w-full"
                                                name="new_password"
                                                id="new_password"
                                                autocomplete="off"
                                                disabled={move || !edit_mode.get() }
                                                prop:value={move || model.get().new_password}
                                                on:input=move |ev| {
                                                    model.update(|curr| {
                                                        curr.new_password = event_target_value(&ev);
                                                    });
                                                }
                                            />
                                        </fieldset>
                                    </div>

                                    // new password (reoeat)
                                    <div class="w-full">
                                        <fieldset class="fieldset">
                                            <label class="label" for="new_password_repeat">"Nueva Contraseña (Repetir)"</label>
                                            <input type="password"
                                                class="input w-full"
                                                name="new_password_repeat"
                                                id="new_password_repeat"
                                                autocomplete="off"
                                                disabled={move || !edit_mode.get() }
                                                prop:value={move || model.get().new_password_repeat}
                                                on:input=move |ev| {
                                                    model.update(|curr| {
                                                        curr.new_password_repeat = event_target_value(&ev);
                                                    });
                                                }
                                            />
                                        </fieldset>
                                    </div>
                                </Show>

                                <button type="submit" class="btn btn-primary w-full" disabled={ move || !edit_mode.get() } >"Guardar"</button>
                            </div>
                        </form>
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
