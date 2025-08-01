use leptos::prelude::*;

use crate::{
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::{email::SendInterestsEmail, interests::get_all_interests},
        models::interest::Interest,
        utils::leptos::{LeptosEmail, LeptosSelectableInterest},
    },
};

#[derive(Default, Debug, Clone, PartialEq)]
enum CurrentDialogPage {
    #[default]
    InterestSelection,
    MailComposing,
}

#[component]
pub fn CategoryEmails() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let current_dialog_page = RwSignal::new(CurrentDialogPage::default());
    let email_model = RwSignal::new(LeptosEmail::default());

    let all_interests = OnceResource::new(get_all_interests());
    let selectable_interests = RwSignal::new(Vec::new());
    Effect::new(move |_| {
        if let Some(Ok(interests)) = all_interests.get() {
            let mut result = vec![];
            for interest in interests {
                result.push(LeptosSelectableInterest {
                    interest,
                    selected: RwSignal::new(false),
                })
            }
            selectable_interests.set(result);
        }
    });

    let send_emails_action = ServerAction::<SendInterestsEmail>::new();
    let send_value = send_emails_action.value();
    Effect::new(move |_| {
        if let Some(val) = send_value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Enviado"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    current_dialog_page.set(CurrentDialogPage::default());
                    email_model.set(LeptosEmail::default());
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
        <div class="card bg-base-100 shadow-xl w-80">
            <div class="card-body w-full h-full justify-between">
                <p class="text-xl text-center">"Escribir Correo por Interés"</p>
                <button class="btn btn-success w-full"
                    on:click=move |_| {
                        let _ = dialog_ref_node.get().unwrap().show_modal();
                    }
                >
                    "Escribir"
                </button>
            </div>
        </div>
        <DialogComponent dialog_title="Enviar Correo" dialog_node_ref=dialog_ref_node dialog_content=move || {
            view! {
                <Suspense fallback=move || view! { <PageLoadingComponent/> }>
                    <ErrorBoundary fallback=|error| view! {
                        <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
                    }>
                        <Show
                            when=move || { !selectable_interests.get().is_empty() }
                            fallback=|| view! { <p class="text-center">"No hay intereses..."</p> }
                        >
                            {
                                if current_dialog_page.get() == CurrentDialogPage::InterestSelection {
                                    view! {
                                        <p class="text-sm font-light text-gray-500 mb-3">"*El correo se enviara a cualquier socio que tenga alguno de los intereses seleccionados"</p>
                                        <div class="flex flex-wrap gap-2 w-full">
                                            <For each=move || selectable_interests.get() key=|i| i.interest.id children=move |i| {
                                                view!{
                                                    <label class="label">
                                                        <input type="checkbox" bind:checked=i.selected class="checkbox checkbox-primary" />
                                                        {i.interest.name}
                                                    </label>
                                                }
                                            }/>
                                        </div>
                                        <div class="mt-3">
                                            <button class="btn btn-accent w-full"
                                                on:click={ move |_|{
                                                    if selectable_interests.get().iter().any(|x| x.selected.get()) {
                                                        current_dialog_page.set(CurrentDialogPage::MailComposing);
                                                    } else {
                                                        set_toast.set(ToastMessage {
                                                            message: String::from("Selecciona al menos un interés"),
                                                            toast_type: ToastType::Error,
                                                            visible: true,
                                                        });
                                                    }
                                                }}
                                            >"Siguiente"</button>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="flex flex-col gap-2 w-full">
                                            <div class="w-full">
                                                <fieldset class="fieldset">
                                                    <label class="label" for="subject">"Asunto"</label>
                                                    <input type="text"
                                                        class="input w-full"
                                                        name="subject"
                                                        id="subject"
                                                        autocomplete="off"
                                                        prop:value={move || email_model.get().subject}
                                                        on:input=move |ev| {
                                                            email_model.update(|curr| {
                                                                curr.subject = event_target_value(&ev);
                                                            });
                                                        }
                                                    />
                                                </fieldset>
                                            </div>

                                            <div class="w-full">
                                                <fieldset class="fieldset">
                                                    <label class="label" for="body">"Contenido"</label>
                                                    <textarea
                                                        class="textarea w-full min-h-40"
                                                        name="body"
                                                        id="body"
                                                        autocomplete="off"
                                                        prop:value={move || email_model.get().body}
                                                        on:input=move |ev| {
                                                            email_model.update(|curr| {
                                                                curr.body = event_target_value(&ev);
                                                            });
                                                        }
                                                    />
                                                </fieldset>
                                            </div>
                                        </div>
                                        <div class="mt-3 flex flex-col gap-2 w-full">
                                            <button class="btn btn-secondary w-full"
                                                on:click={ move |_|{
                                                    current_dialog_page.set(CurrentDialogPage::InterestSelection);
                                                }}
                                            >"Atrás"</button>
                                            <button class="btn btn-success w-full"
                                                on:click={ move |_|{
                                                    let email = email_model.get();
                                                    let interests: Vec<Interest> = selectable_interests.get().iter()
                                                        .filter(|x| x.selected.get())
                                                        .map(|x| x.interest.clone())
                                                        .collect();

                                                    send_emails_action.dispatch(SendInterestsEmail { interests, subject: email.subject, original_body: email.body });
                                                }}
                                            >"Enviar"</button>
                                        </div>
                                    }.into_any()
                                }
                            }
                        </Show>
                    </ErrorBoundary>
                </Suspense>
            }
        }/>
    }
}
