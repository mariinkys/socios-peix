use leptos::prelude::*;
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
};

use crate::{
    components::{
        dialog::DialogComponent,
        emails::member_emails::MemberEmails,
        interests::member_interests_upsert::MemberInterestsUpsert,
        members::upsert::UpsertMember,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::members::{DeleteMember, get_member},
        models::member::Member,
    },
};

#[derive(Params, PartialEq)]
struct MemberParams {
    id: Option<i32>,
}

#[component]
pub fn UpsertMemberPage() -> impl IntoView {
    let params = use_params::<MemberParams>();
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let member_model = RwSignal::new(Member::default());
    let edit_mode = RwSignal::new(false);

    let member_resource = Resource::new(
        move || params.read().as_ref().ok().and_then(|params| params.id),
        move |params| async move {
            match params {
                Some(id) => get_member(id).await,
                None => {
                    edit_mode.set(true);
                    Ok(Member::default())
                }
            }
        },
    );

    Effect::new(move |_| {
        if let Some(resource_result) = member_resource.get() {
            match resource_result {
                Ok(member) => member_model.set(member),
                Err(_) => member_model.set(Member::default()),
            }
        }
    });

    let delete_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let delete_action = ServerAction::<DeleteMember>::new();
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

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <div class="flex flex-row gap-2 mb-3 items-center">
                    <h2 class="text-2xl grow">"Detalles Socio"</h2>
                    <button
                        class="btn"
                        class:btn-accent=move || !edit_mode.get()
                        disabled=move || member_model.get().id.is_none()
                        on:click=move |_| edit_mode.update(|val| *val = !*val)
                    >
                        "Editar"
                    </button>
                    <button
                        class="btn btn-error"
                        disabled=move || member_model.get().id.is_none()
                        on:click=move |_| {
                            let _ = delete_dialog_ref_node.get().unwrap().show_modal();
                    }>
                        "Eliminar"
                    </button>
                </div>

                <DialogComponent dialog_title="Eliminar Socio" dialog_node_ref=delete_dialog_ref_node dialog_content=move || {
                    view! {
                        <div>
                            <p class="text-center font-bold text-xl text-error">"Seguro que quieres eliminar a este socio?"</p>
                            <p class="text-center font-bold text-xl text-error">"Esta acción es irreversible"</p>
                        </div>

                        <button class="btn btn-error w-full"
                            on:click=move |_| {
                                if let Some(member_id) = member_model.get().id {
                                    delete_action.dispatch(DeleteMember { member_id });
                                }
                        }>
                        "Delete"</button>
                    }
                }/>

                <div class="flex flex-col md:flex-row w-full gap-2 h-full md:h-[80vh]">
                    <UpsertMember edit_mode=edit_mode model=member_model/>
                    <Show
                        when=move || { member_model.get().id.is_some() }
                        fallback=|| view! { <p></p> }
                    >
                        <MemberInterestsUpsert edit_mode=edit_mode member_id=member_model.get_untracked().id.unwrap()/>
                    </Show>
                </div>
                <div class="mt-2">
                    <Show
                        when=move || { member_model.get().id.is_some() }
                        fallback=|| view! { <p></p> }
                    >
                        <MemberEmails edit_mode=edit_mode member_id=member_model.get_untracked().id.unwrap()/>
                    </Show>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
