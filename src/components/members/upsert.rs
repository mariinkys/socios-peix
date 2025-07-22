use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::{
    components::{
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::members::UpsertMember,
        entities::{country::Country, gender::Gender},
        models::member::Member,
    },
};

#[component]
pub fn UpsertMember(edit_mode: RwSignal<bool>, model: RwSignal<Member>) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let upsert_member_action = ServerAction::<UpsertMember>::new();
    let upsert_value = upsert_member_action.value();
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
                    navigate("/members", Default::default());
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
        let member_model = model.get();

        if member_model.is_valid() {
            upsert_member_action.dispatch(UpsertMember {
                member: member_model,
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
                            <h2 class="card-title grow">"Socio"</h2>
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

                                // surname
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="surname">"Primer Apellido*"</label>
                                        <input type="text"
                                            class="input w-full"
                                            name="surname"
                                            id="surname"
                                            required
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value={move || model.get().surname}
                                            on:input=move |ev| {
                                                model.update(|curr| {
                                                    curr.surname = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                // second surname
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="second_surname">"Segundo Apellido"</label>
                                        <input type="text"
                                            class="input w-full"
                                            name="second_surname"
                                            id="second_surname"
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value={move || model.get().second_surname}
                                            on:input=move |ev| {
                                                model.update(|curr| {
                                                    curr.second_surname = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                // email
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="email">"Correo Electrónico"</label>
                                        <input type="text"
                                            class="input w-full"
                                            name="email"
                                            id="email"
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value={move || model.get().email}
                                            on:input=move |ev| {
                                                model.update(|curr| {
                                                    curr.email = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                // birthdate
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="birthdate">"Fecha de Nacimiento"</label>
                                        <input type="date"
                                            class="input w-full"
                                            name="birthdate"
                                            id="birthdate"
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value=move || {
                                                model.get().birthdate
                                                    .map(|date| date.format("%Y-%m-%d").to_string())
                                                    .unwrap_or_default()
                                                }
                                            on:input=move |ev| {
                                                let value = event_target_value(&ev);
                                                model.update(|curr| {
                                                    if value.is_empty() {
                                                        curr.birthdate = None;
                                                    } else {
                                                        curr.birthdate = chrono::NaiveDate::parse_from_str(&value, "%Y-%m-%d").ok();
                                                    }
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                // phone
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="phone">"Móvil / Teléfono"</label>
                                        <input type="text"
                                            class="input w-full"
                                            name="phone"
                                            id="phone"
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value={move || model.get().phone}
                                            on:input=move |ev| {
                                                model.update(|curr| {
                                                    curr.phone = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                //country
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="country_id">"País*"</label>
                                        <select
                                            class="select w-full"
                                            name="country_id"
                                            id="country_id"
                                            required
                                            disabled={move || !edit_mode.get() }
                                            on:change=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                    model.update(|curr| {
                                                        curr.country = Country::from_id(val).unwrap_or_default();
                                                    });
                                                }
                                            }
                                            prop:value={move || Country::to_id(model.get().country)}
                                        >
                                            <For each=move || Country::ALL key=|c| c.to_id() children=move |country| {
                                                view!{
                                                    <option value=country.to_id()>{country.to_string()}</option>
                                                }
                                            }/>
                                        </select>
                                    </fieldset>
                                </div>

                                //gender
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="gender_id">"Género*"</label>
                                        <select
                                            class="select w-full"
                                            name="gender_id"
                                            id="gender_id"
                                            required
                                            disabled={move || !edit_mode.get() }
                                            on:change=move |ev| {
                                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                    model.update(|curr| {
                                                        curr.gender = Gender::from_id(val).unwrap_or_default();
                                                    });
                                                }
                                            }
                                            prop:value={move || Gender::to_id(model.get().gender)}
                                        >
                                            <For each=move || Gender::ALL key=|g| g.to_id() children=move |gender| {
                                                view!{
                                                    <option value=gender.to_id()>{gender.to_string()}</option>
                                                }
                                            }/>
                                        </select>
                                    </fieldset>
                                </div>

                                //notes
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="notes">"Observaciones"</label>
                                        <textarea
                                            class="textarea w-full min-h-80"
                                            name="notes"
                                            id="notes"
                                            autocomplete="off"
                                            disabled={move || !edit_mode.get() }
                                            prop:value={move || model.get().notes}
                                            on:input=move |ev| {
                                                model.update(|curr| {
                                                    curr.notes = event_target_value(&ev);
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
