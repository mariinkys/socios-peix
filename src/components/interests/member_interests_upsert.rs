use leptos::prelude::*;

use crate::{
    components::{
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    core::{
        api::interests::{get_member_interests, UpdateMemberInterest},
        models::interest::Interest,
    },
};

#[derive(Clone)]
struct LeptosSelectableInterest {
    interest: Interest,
    selected: RwSignal<bool>,
}

#[component]
pub fn MemberInterestsUpsert(edit_mode: RwSignal<bool>, member_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let selectable_interests = RwSignal::new(Vec::<LeptosSelectableInterest>::new());
    let member_interests = Resource::new(
        move || member_id,
        |member_id| async move { get_member_interests(member_id).await },
    );
    Effect::new(move |_| {
        if let Some(Ok(interests)) = member_interests.get() {
            let mut all = vec![];
            for interest in interests {
                all.push(LeptosSelectableInterest {
                    interest: interest.interest,
                    selected: RwSignal::new(interest.is_selected),
                });
            }
            selectable_interests.set(all);
        }
    });

    let upsert_client_interest_action = ServerAction::<UpdateMemberInterest>::new();
    let upsert_value = upsert_client_interest_action.value();
    Effect::new(move |_| {
        if let Some(val) = upsert_value.get() {
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

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // Stop the page from reloading
        ev.prevent_default();

        let interests_to_update = selectable_interests.get();
        let mut all = vec![];
        for interest in interests_to_update {
            all.push(crate::core::models::interest::SelectableInterest {
                interest: interest.interest,
                is_selected: interest.selected.get_untracked(),
            })
        }

        upsert_client_interest_action.dispatch(UpdateMemberInterest {
            member_id,
            selected_interests: all,
        });
    };

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
                <Show
                    when=move || { member_interests.get().is_some_and(|x| x.is_ok_and(|y| !y.is_empty()))}
                    fallback=|| view! { <p class="text-center">"No hay intereses..."</p> }
                >
                    <div class="card card-border bg-base-200 w-full">
                        <div class="card-body">
                            <div class="flex gap-1">
                                <h2 class="card-title grow">"Intereses"</h2>
                            </div>
                            <form class="w-full" on:submit=on_submit>
                                <div class="flex flex-wrap gap-2">
                                    <For each=move || selectable_interests.get() key=|i| i.interest.id children=move |i| {
                                        let is_selected = i.selected;
                                        let name = i.interest.name;

                                        view! {
                                            <fieldset class="fieldset bg-base-100 border-base-300 rounded-box w-full sm:w-auto border p-4">
                                                <label class="label">
                                                    <input type="checkbox"
                                                        checked=move || is_selected.get()
                                                        disabled={move || !edit_mode.get() }
                                                        bind:checked=is_selected
                                                        class="checkbox" />
                                                    {name}
                                                </label>
                                            </fieldset>
                                        }
                                    }/>
                                </div>
                                <button type="submit" class="btn btn-primary w-full mt-3" disabled={ move || !edit_mode.get() } >"Guardar"</button>
                            </form>
                        </div>
                    </div>
                </Show>
            </ErrorBoundary>
        </Suspense>
    }
}
