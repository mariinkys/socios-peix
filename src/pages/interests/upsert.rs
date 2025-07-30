use leptos::prelude::*;
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
};

use crate::{
    components::{
        dialog::DialogComponent,
        interests::upsert::UpsertInterest,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::interests::{DeleteInterest, get_interest},
        models::interest::Interest,
    },
};

#[derive(Params, PartialEq)]
struct InterestParams {
    id: Option<i32>,
}

#[component]
pub fn UpsertInterestPage() -> impl IntoView {
    let params = use_params::<InterestParams>();
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let interest_model = RwSignal::new(Interest::default());
    let edit_mode = RwSignal::new(false);

    let interest_resource = Resource::new(
        move || params.read().as_ref().ok().and_then(|params| params.id),
        move |params| async move {
            match params {
                Some(id) => get_interest(id).await,
                None => {
                    edit_mode.set(true);
                    Ok(Interest::default())
                }
            }
        },
    );

    Effect::new(move |_| {
        if let Some(resource_result) = interest_resource.get() {
            match resource_result {
                Ok(interest) => interest_model.set(interest),
                Err(_) => interest_model.set(Interest::default()),
            }
        }
    });

    let delete_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let delete_action = ServerAction::<DeleteInterest>::new();
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

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="flex flex-row gap-2 mb-3 items-center">
                    <h2 class="text-2xl grow">"Detalles Interés"</h2>
                    <button
                        class="btn"
                        class:btn-accent=move || !edit_mode.get()
                        class:btn-error=move || edit_mode.get()
                        disabled=move || interest_model.get().id.is_none()
                        on:click=move |_| edit_mode.update(|val| *val = !*val)
                    >
                        "Editar"
                    </button>
                    <button
                        class="btn btn-error"
                        disabled=move || interest_model.get().id.is_none()
                        on:click=move |_| {
                            let _ = delete_dialog_ref_node.get().unwrap().show_modal();
                    }>
                        "Eliminar"
                    </button>
                </div>

                <DialogComponent dialog_title="Eliminar Interés" dialog_node_ref=delete_dialog_ref_node dialog_content=move || {
                    view! {
                        <div>
                            <p class="text-center font-bold text-xl text-error">"Seguro que quieres eliminar este interés?"</p>
                            <p class="text-center font-bold text-xl text-error">"Esta acción es irreversible"</p>
                        </div>

                        <button class="btn btn-error w-full"
                            on:click=move |_| {
                                if let Some(interest_id) = interest_model.get().id {
                                    delete_action.dispatch(DeleteInterest { interest_id });
                                }
                        }>
                        "Delete"</button>
                    }
                }/>

                <UpsertInterest edit_mode=edit_mode model=interest_model/>
            </ErrorBoundary>
        </Suspense>
    }
}
