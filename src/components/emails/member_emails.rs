use leptos::prelude::*;

use crate::{
    components::{
        dialog::DialogComponent,
        emails::LeptosEmail,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::api::email::{SendSingleEmail, get_member_emails},
};

#[component]
pub fn MemberEmails(edit_mode: RwSignal<bool>, member_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let model = RwSignal::new(LeptosEmail::default());

    let member_emails = Resource::new(
        move || member_id,
        |member_id| async move { get_member_emails(member_id).await },
    );

    let send_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let send_email_action = ServerAction::<SendSingleEmail>::new();
    let send_value = send_email_action.value();
    Effect::new(move |_| {
        if let Some(val) = send_value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Enviado"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    model.set(LeptosEmail::default());
                    member_emails.refetch();
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
                <div class="card card-border bg-base-200 w-full">
                    <div class="card-body">
                        <div class="flex gap-1">
                            <h2 class="card-title grow">"Emails"</h2>
                            <button
                                class="btn btn-accent"
                                disabled={move || !edit_mode.get() }
                                on:click=move |_| {
                                    let _ = send_dialog_ref_node.get().unwrap().show_modal();
                                }
                            >"Enviar"</button>
                            <DialogComponent dialog_title="Enviar Correo" dialog_node_ref=send_dialog_ref_node dialog_content=move || {
                                view! {
                                    <ActionForm action=send_email_action>
                                        <div class="flex flex-col gap-2 w-full">
                                            // We need the id for the update but we don't want to show it.
                                            <input type="hidden" name="member_id" autocomplete="off"  prop:value={member_id}/>
                                            <div class="w-full">
                                                <fieldset class="fieldset">
                                                    <label class="label" for="subject">"Asunto"</label>
                                                    <input type="text"
                                                        class="input w-full"
                                                        name="subject"
                                                        id="subject"
                                                        autocomplete="off"
                                                        disabled={move || !edit_mode.get() }
                                                        prop:value={move || model.get().subject}
                                                        on:input=move |ev| {
                                                            model.update(|curr| {
                                                                curr.subject = event_target_value(&ev);
                                                            });
                                                        }
                                                    />
                                                </fieldset>
                                            </div>

                                            <div class="w-full">
                                                <fieldset class="fieldset">
                                                    <label class="label" for="original_body">"Contenido"</label>
                                                    <textarea
                                                        class="textarea w-full min-h-40"
                                                        name="original_body"
                                                        id="original_body"
                                                        autocomplete="off"
                                                        disabled={move || !edit_mode.get() }
                                                        prop:value={move || model.get().body}
                                                        on:input=move |ev| {
                                                            model.update(|curr| {
                                                                curr.body = event_target_value(&ev);
                                                            });
                                                        }
                                                    />
                                                </fieldset>
                                            </div>

                                            <button disabled={ move || !edit_mode.get() } type="submit" class="btn btn-primary">"Enviar"</button>
                                        </div>
                                    </ActionForm>
                                }
                            }/>
                        </div>
                        <div class="flex flex-col gap-2">
                            <Show
                                when=move || { member_emails.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                                fallback=|| view! { <p class="text-center">"No hay emails..."</p> }
                            >
                                <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-200">
                                    <table class="table">
                                        <thead>
                                        <tr>
                                            <th></th>
                                            <th>"Asunto"</th>
                                            <th>"Contenido"</th>
                                            <th>"Enviado el Día"</th>
                                        </tr>
                                        </thead>
                                        <tbody>
                                            <For each=move || member_emails.get().and_then(|res| res.ok()).unwrap_or_default() key=|e| e.id children=move |e| {
                                                view! {
                                                    <tr>
                                                        <th>{e.id}</th>
                                                        <td>{e.subject}</td>
                                                        <td>{e.body}</td>
                                                        <td>{e.created_at.map(|x| x.format("%d-%m-%Y").to_string()).unwrap_or_else(|| "N/A".to_string())}</td>
                                                    </tr>
                                                }
                                            }/>
                                        </tbody>
                                    </table>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
