use leptos::prelude::*;

use crate::{
    components::{
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::api::users::Login,
};

#[component]
pub fn LoginPage() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());

    let login_action = ServerAction::<Login>::new();
    let login_value = login_action.value();
    Effect::new(move |_| {
        if let Some(val) = login_value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Inicio de Sesión Válido"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    if let Some(win) = web_sys::window() {
                        win.location().reload().unwrap();
                    }
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

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="min-h-[95vh] w-full flex items-center justify-center">
                    <div class="card card-border bg-base-200 w-full xl:w-1/2 m-auto rounded-xl">
                        <div class="card-body">
                            <ActionForm action=login_action>
                                <div class="flex gap-4 items-center">
                                    <div class="hidden xl:block">
                                        <img src="/assets/main.webp"
                                            alt="Hotel Photo"
                                            class="object-cover rounded-xl shadow-md"
                                            loading="lazy"/>
                                    </div>
                                    <div class="flex flex-col gap-2 w-full">
                                        <p class="m-auto text-3xl font-bold text-center">"Inicio de Sesión"</p>
                                        <div class="w-full">
                                            <fieldset class="fieldset">
                                                <label class="label" for="username">"Nombre de Usuario"</label>
                                                <input type="text"
                                                    class="input w-full"
                                                    name="username"
                                                    id="username"
                                                    autocomplete="off"
                                                    prop:value={move || username.get()}
                                                    on:input=move |ev| {
                                                        username.update(|curr| {
                                                            *curr = event_target_value(&ev);
                                                        });
                                                    }
                                                />
                                            </fieldset>
                                        </div>

                                        <div class="w-full">
                                            <fieldset class="fieldset">
                                                <label class="label" for="password">"Contraseña"</label>
                                                <input type="password"
                                                    class="input w-full"
                                                    name="password"
                                                    id="password"
                                                    autocomplete="off"
                                                    prop:value={move || password.get()}
                                                    on:input=move |ev| {
                                                        password.update(|curr| {
                                                            *curr = event_target_value(&ev);
                                                        });
                                                    }
                                                />
                                            </fieldset>
                                        </div>

                                        <button disabled={move || password.get().is_empty() || username.get().is_empty()} type="submit" class="btn btn-primary">"Iniciar Sesión"</button>
                                    </div>
                                </div>
                            </ActionForm>
                        </div>
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
