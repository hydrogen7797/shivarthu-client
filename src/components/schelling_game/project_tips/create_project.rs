use crate::components::api::ipfs_request::ipfs_call_json_string;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;
use crate::components::markdown::markdown_field::MarkdownField;
use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::project_tips::create_project_sign_in::SignTransaction;
use crate::services::common_imp::View;
use json::object;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum ErrorHandling {
    #[error("Not a tip")]
    NotTip,
}

async fn get_cid_post(
    details: String,
    set_current_view: WriteSignal<View>,
    set_post_cid: WriteSignal<String>,
) {
    let data = object! {
          version: "1.0",
          details: details,
    };
    let json_string = json::stringify(data);
    let response =
        ipfs_call_json_string(DEFAULT_IPFS_PROVIDER, &json_string, "ipfs".to_owned()).await;
    set_post_cid(response);
    set_current_view(View::Success);
}

#[component]
pub fn SelectOption(is: &'static str, tip_name: ReadSignal<String>) -> impl IntoView {
    view! {
        <option value={is} selected={move || tip_name() == is}>
            {is}
        </option>
    }
}

#[component]
pub fn CreateProject() -> impl IntoView {
    let params = use_params_map();

    let department_id_fn = move || {
        params.with(|params| {
            params
                .get("department_id")
                .cloned()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or_default()
        })
    };

    let department_id = untrack(move || department_id_fn());

    let (current_view, set_current_view) = create_signal(View::Form);
    let (markdown, set_markdown) = create_signal(String::from(""));
    let (post_cid, set_post_cid) = create_signal(String::from(""));
    let (tip_name, set_tip_name) = create_signal(String::from(""));
    let (funding_needed, set_funding_needed) = create_signal::<Option<u128>>(None);

    let submit_action = create_action(
        |(details, set_current_view, set_post_cid): &(
            String,
            WriteSignal<View>,
            WriteSignal<String>,
        )| {
            let details = details.to_owned();
            let set_current_view = set_current_view.clone();
            let set_post_cid = set_post_cid.clone();

            async move { get_cid_post(details, set_current_view, set_post_cid).await }
        },
    );
    let _submitted = submit_action.input();
    let pending = submit_action.pending();
    let submit_action_value = submit_action.value();

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        submit_action.dispatch((markdown(), set_current_view, set_post_cid));
    };

    let funding_needed_changed = move |value: String| {
        let choice_value = value.parse::<u128>().expect("Invalid input");
        gloo::console::log!(choice_value);
        set_funding_needed(Some(choice_value));
    };

    let cid_value = move || {
        submit_action_value();
    };

    let render_view = move || match current_view() {
        View::Form =>
        // if post_cid().is_empty() {
        {
            view! {
                <div>
                    <form
                        class="max-w-5xl mx-auto max-md:mx-10"
                        id="add-profile-submit-from"
                        on:submit={submit_click}
                    >
                        <div class="mb-5">
                            <label
                                for="profile-details"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Profile Details
                            </label>
                            <MarkdownField
                                set_markdown={set_markdown}
                                name={String::from("profile-details")}
                                class={String::from(
                                    "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                )}
                            />

                        </div>

                        <select
                            on:change={move |ev| {
                                let new_value = event_target_value(&ev);
                                set_tip_name(new_value);
                            }}

                            id="tipperselect"
                            class="select select-info w-full max-w-xs"
                        >
                            <option disabled selected>
                                Select the tipper
                            </option>
                            <SelectOption tip_name is="SmallTipper"/>
                            <SelectOption tip_name is="BigTipper"/>
                            <SelectOption tip_name is="SmallSpender"/>
                            <SelectOption tip_name is="MediumSpender"/>
                            <SelectOption tip_name is="BigSpender"/>
                        </select>

                        <div class="mb-5">
                            <label
                                for="funding-needed"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Funding Needed
                            </label>
                            <input
                                type="number"
                                id="funding-needed"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input={move |ev| funding_needed_changed(event_target_value(&ev))}
                            />
                        </div>

                        <br/>
                        <br/>
                        <button
                            type="submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Submit
                        </button>

                    </form>
                    <p>{move || pending().then(|| "Loading...")}</p>
                    <p>{move || cid_value()}</p>
                </div>
            }
        }

        View::Success => view! {
            <div>
                <SignTransaction
                    post_cid={post_cid()}
                    department_id={department_id}
                    tip_name={tip_name()}
                    funding_needed={funding_needed().unwrap()}
                />
            </div>
        },
    };

    view! {
        <>
            <Nav/>
            {move || render_view()}
        </>
    }
}
