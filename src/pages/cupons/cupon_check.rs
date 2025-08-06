use leptos::prelude::*;

use crate::{
    components::{
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{api::cupons::CuponsByCode, models::cupon::Cupon},
};

#[component]
pub fn CuponCheckPage() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let cupons_model = RwSignal::new(Vec::<Cupon>::new());

    let cupon_input = RwSignal::new(String::new());
    let search_by_code = ServerAction::<CuponsByCode>::new();
    let search_value = search_by_code.value();
    Effect::new(move |_| {
        if let Some(val) = search_value.get() {
            match val {
                Ok(res) => {
                    if res.is_empty() {
                        set_toast.set(ToastMessage {
                            message: String::from("No existe o esta caducado!"),
                            toast_type: ToastType::Error,
                            visible: true,
                        });
                    } else {
                        cupons_model.set(res);
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
                <div class="flex flex-row gap-2 mb-3 items-center">
                    <h2 class="text-2xl grow">"Comprobar Cupón"</h2>
                </div>

                <div class="flex justify-center w-full items-center">
                    <div class="flex flex-col md:flex-row gap-2 w-full md:w-1/2 items-center">
                        <fieldset class="fieldset w-full flex-1">
                            <input type="text"
                                class="input w-full"
                                name="subject"
                                id="subject"
                                placeholder="Cupón"
                                autocomplete="off"
                                prop:value={move || cupon_input.get()}
                                on:input=move |ev| {
                                    cupon_input.set(event_target_value(&ev));
                                }
                            />
                        </fieldset>

                        <div class="flex md:items-end w-full md:w-auto">
                            <button class="btn btn-primary w-full md:w-auto"
                                on:click={ move |_|{
                                    let cupon = cupon_input.get();
                                    search_by_code.dispatch(CuponsByCode { cupon });
                                }}
                            >
                                "Buscar"
                            </button>
                        </div>
                    </div>
                </div>

                <Show
                    when=move || { !cupons_model.get().is_empty() }
                    fallback=|| view! { <p class="text-center mt-3">"No hay cupones..."</p> }
                >
                    <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200 mt-3">
                        <table class="table">
                            <thead>
                            <tr>
                                <th></th>
                                <th>"Cupón"</th>
                                <th>"Descripción"</th>
                                <th>"Enviado el Día"</th>
                                <th>"Caduca el Día"</th>
                                <th>"Usado"</th>
                            </tr>
                            </thead>
                            <tbody>
                                <For each=move || cupons_model.get() key=|c| c.id children=move |c| {
                                    view! {
                                        <tr>
                                            <th>{c.id}</th>
                                            <td>{c.code}</td>
                                            <td>{c.description}</td>
                                            <td>{c.created_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                            <td>{c.expires_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                            <td>{if c.used { "Sí" } else { "No" }}</td>
                                        </tr>
                                    }
                                }/>
                            </tbody>
                        </table>
                    </div>
                </Show>
            </ErrorBoundary>
        </Suspense>
    }
}
