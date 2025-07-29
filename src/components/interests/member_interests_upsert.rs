use leptos::prelude::*;

use crate::{
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::api::interests::{
        AddMemberInterest, RemoveMemberInterest, get_all_interests, get_member_interests,
    },
};

#[component]
pub fn MemberInterestsUpsert(edit_mode: RwSignal<bool>, member_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let all_interests = Resource::new(|| (), |_| get_all_interests());
    let member_interests = Resource::new(
        move || member_id,
        |member_id| async move { get_member_interests(member_id).await },
    );
    let selected_add_interest_id = RwSignal::new(0);
    Effect::new(move |_| {
        #[allow(clippy::collapsible_if)]
        if let Some(Ok(interests)) = all_interests.get() {
            if !interests.is_empty() {
                selected_add_interest_id.set(interests.first().unwrap().id.unwrap_or_default())
            }
        }
    });

    let add_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let add_member_interest_action = ServerAction::<AddMemberInterest>::new();
    let add_value = add_member_interest_action.value();
    Effect::new(move |_| {
        if let Some(val) = add_value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Guardado"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    member_interests.refetch();
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

    let delete_member_interest_action = ServerAction::<RemoveMemberInterest>::new();
    let delete_value = delete_member_interest_action.value();
    Effect::new(move |_| {
        if let Some(val) = delete_value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Borrado"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    member_interests.refetch();
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
                <div class="basis-1/3 card card-border bg-base-200 w-full overflow-y-auto">
                    <div class="card-body">
                        <div class="flex gap-1">
                            <h2 class="card-title grow">"Intereses"</h2>
                            <button
                                class="btn btn-accent"
                                disabled={move || !edit_mode.get() }
                                on:click=move |_| {
                                    let _ = add_dialog_ref_node.get().unwrap().show_modal();
                                }
                            >"Añadir"</button>
                            <DialogComponent dialog_title="Añadir Interés" dialog_node_ref=add_dialog_ref_node dialog_content=move || {
                                view! {
                                    <Show
                                        when=move || { all_interests.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                                        fallback=|| view! { <p class="text-center">"No hay intereses..."</p> }
                                    >
                                        <ActionForm action=add_member_interest_action>
                                            <div class="flex flex-col gap-2 w-full">
                                                // We need the id for the update but we don't want to show it.
                                                <input type="hidden" name="member_id" autocomplete="off"  prop:value={member_id}/>
                                                <div class="w-full">
                                                    <fieldset class="fieldset">
                                                        <label class="label" for="interest_id">"Interés"</label>
                                                        <select
                                                            class="select w-full"
                                                            name="interest_id"
                                                            id="interest_id"
                                                            required
                                                            on:change=move |ev| {
                                                                if let Ok(val) = event_target_value(&ev).parse::<i32>() {
                                                                    selected_add_interest_id.update(|curr| {
                                                                        *curr = val;
                                                                    });
                                                                }
                                                            }
                                                            prop:value={move || selected_add_interest_id.get()}
                                                        >
                                                            <For each=move || all_interests.get().and_then(|res| res.ok()).unwrap_or_default() key=|i| i.id children=move |interest| {
                                                                view!{
                                                                    <option value=interest.id>{interest.name}</option>
                                                                }
                                                            }/>
                                                        </select>
                                                        <button disabled={ move || !edit_mode.get() } type="submit" class="btn btn-primary">"Añadir"</button>
                                                    </fieldset>
                                                </div>
                                            </div>
                                        </ActionForm>
                                    </Show>
                                }
                            }/>
                        </div>
                        <div class="flex flex-col gap-2">
                            <Show
                                when=move || { member_interests.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                                fallback=|| view! { <p class="text-center">"No hay intereses..."</p> }
                            >
                                <For each=move || member_interests.get().and_then(|res| res.ok()).unwrap_or_default() key=|i| i.id children=move |i| {
                                    view! {
                                        <ActionForm action=delete_member_interest_action>
                                            <div class="card card-border bg-base-300 w-full">
                                                <div class="card-body p-3">
                                                    <input type="hidden" name="member_id" autocomplete="off" prop:value={member_id}/>
                                                    <input type="hidden" name="interest_id" autocomplete="off" prop:value={i.id.unwrap_or_default()}/>
                                                    <div class="flex gap-1">
                                                        <h3 class="card-title grow">{i.name}</h3>
                                                        <button disabled={ move || !edit_mode.get() } type="submit" class="btn btn-error">"Borrar"</button>
                                                    </div>
                                                </div>
                                            </div>
                                        </ActionForm>
                                    }
                                }/>
                            </Show>
                        </div>
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
