use leptos::prelude::*;
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
};

use crate::{
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
        users::upsert::UpsertUser,
    },
    core::{
        api::users::{ChangeUserPassword, DeleteUser, get_user},
        models::user::{UpsertOperation, UserUpsertModel},
    },
};

#[derive(Params, PartialEq)]
struct UserParams {
    id: Option<i32>,
}

#[component]
pub fn UpsertUserPage() -> impl IntoView {
    let params = use_params::<UserParams>();
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let user_model = RwSignal::new(UserUpsertModel::default());
    let edit_mode = RwSignal::new(false);

    let user_resource = Resource::new(
        move || params.read().as_ref().ok().and_then(|params| params.id),
        move |params| async move {
            match params {
                Some(id) => get_user(id).await,
                None => {
                    edit_mode.set(true);
                    Ok(UserUpsertModel::default())
                }
            }
        },
    );

    Effect::new(move |_| {
        if let Some(resource_result) = user_resource.get() {
            match resource_result {
                Ok(user) => user_model.set(user),
                Err(_) => user_model.set(UserUpsertModel::default()),
            }
        }
    });

    let delete_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let delete_action = ServerAction::<DeleteUser>::new();
    let delete_value = delete_action.value();
    Effect::new(move |_| {
        if let Some(val) = delete_value.get() {
            match val {
                Ok(_) => {
                    let navigate = use_navigate();
                    set_toast.set(ToastMessage {
                        message: String::from("Eliminado"),
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

    let change_password_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let change_password_action = ServerAction::<ChangeUserPassword>::new();
    let change_password_value = change_password_action.value();
    Effect::new(move |_| {
        if let Some(val) = change_password_value.get() {
            match val {
                Ok(_) => {
                    change_password_dialog_ref_node.get().unwrap().close();
                    set_toast.set(ToastMessage {
                        message: String::from("Cambiada Correctamente!"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    user_resource.refetch();
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
        let user_model = user_model.get();
        let operation = UpsertOperation::PasswordChange;

        if user_model.is_valid(operation) {
            change_password_action.dispatch(ChangeUserPassword { user: user_model });
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
                <div class="flex flex-col md:flex-row md:justify-between gap-4 md:gap-2 mb-3 items-cemter">
                    <h2 class="text-2xl grow">"Detalles Usuario"</h2>
                    <div class="flex flex-col md:flex-row gap-2 w-full md:w-auto">
                        <button
                            class="btn w-full md:w-auto"
                            class:btn-accent=move || !edit_mode.get()
                            class:btn-error=move || edit_mode.get()
                            disabled=move || user_model.get().id.is_none()
                            on:click=move |_| edit_mode.update(|val| *val = !*val)
                        >
                            "Editar"
                        </button>
                        <button
                            class="btn btn-accent w-full md:w-auto"
                            disabled=move || user_model.get().id.is_none()
                            on:click=move |_| {
                                let _ = change_password_dialog_ref_node.get().unwrap().show_modal();
                        }>
                            "Cambiar Contraseña"
                        </button>
                        <button
                            class="btn btn-error w-full md:w-auto"
                            disabled=move || user_model.get().id.is_none()
                            on:click=move |_| {
                                let _ = delete_dialog_ref_node.get().unwrap().show_modal();
                        }>
                            "Eliminar"
                        </button>
                    </div>
                </div>

                <DialogComponent dialog_title="Eliminar Usuario" dialog_node_ref=delete_dialog_ref_node dialog_content=move || {
                    view! {
                        <div>
                            <p class="text-center font-bold text-xl text-error">"Seguro que quieres eliminar este usuario?"</p>
                            <p class="text-center font-bold text-xl text-error">"Esta acción es irreversible"</p>
                        </div>

                        <button class="btn btn-error w-full"
                            on:click=move |_| {
                                if let Some(user_id) = user_model.get().id {
                                    delete_action.dispatch(DeleteUser { user_id });
                                }
                        }>
                        "Delete"</button>
                    }
                }/>

                <DialogComponent dialog_title="Cambiar Contraseña" dialog_node_ref=change_password_dialog_ref_node dialog_content=move || {
                    view! {
                        <form class="w-full" on:submit=on_submit>
                            <div class="flex flex-col gap-2 w-full">
                                //old password (if any)
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="old_password">"Antigua Contraseña"</label>
                                        <input type="password"
                                            class="input w-full"
                                            name="old_password"
                                            id="old_password"
                                            autocomplete="off"
                                            prop:value={move || user_model.get().old_password}
                                            on:input=move |ev| {
                                                user_model.update(|curr| {
                                                    curr.old_password = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                // new password
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="new_password">"Nueva Contraseña"</label>
                                        <input type="password"
                                            class="input w-full"
                                            name="new_password"
                                            id="new_password"
                                            autocomplete="off"
                                            prop:value={move || user_model.get().new_password}
                                            on:input=move |ev| {
                                                user_model.update(|curr| {
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
                                            prop:value={move || user_model.get().new_password_repeat}
                                            on:input=move |ev| {
                                                user_model.update(|curr| {
                                                    curr.new_password_repeat = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                <button type="submit" class="btn btn-primary w-full">"Cambiar"</button>
                            </div>
                        </form>
                    }
                }/>

                <UpsertUser edit_mode=edit_mode model=user_model/>
            </ErrorBoundary>
        </Suspense>
    }
}
