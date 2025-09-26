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
        api::users::{DeleteUser, get_user},
        models::user::UserUpsertModel,
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

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="flex flex-row gap-2 mb-3 items-center">
                    <h2 class="text-2xl grow">"Detalles Usuario"</h2>
                    <button
                        class="btn"
                        class:btn-accent=move || !edit_mode.get()
                        class:btn-error=move || edit_mode.get()
                        disabled=move || user_model.get().id.is_none()
                        on:click=move |_| edit_mode.update(|val| *val = !*val)
                    >
                        "Editar"
                    </button>
                    <button
                        class="btn btn-error"
                        disabled=move || user_model.get().id.is_none()
                        on:click=move |_| {
                            let _ = delete_dialog_ref_node.get().unwrap().show_modal();
                    }>
                        "Eliminar"
                    </button>
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

                <UpsertUser edit_mode=edit_mode model=user_model/>
            </ErrorBoundary>
        </Suspense>
    }
}
