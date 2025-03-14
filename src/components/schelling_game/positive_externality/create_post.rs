use crate::components::api::ipfs_request::ipfs_call_json_string;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;
use crate::components::markdown::markdown_field::MarkdownField;
use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::positive_externality::create_post_sign_in::SignTransaction;
use crate::services::common_imp::View;
use json::object;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

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
pub fn CreatePositiveExternalityPost() -> impl IntoView {
    let (current_view, set_current_view) = signal(View::Form);
    let (markdown, set_markdown) = signal(String::from(""));
    let (post_cid, set_post_cid) = signal(String::from(""));

    let submit_action: Action<(String, WriteSignal<View>, WriteSignal<String>), (), LocalStorage> =
        Action::new_unsync(
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

    let cid_value = move || {
        submit_action_value();
    };

    let render_view = move || {
        match current_view() {
        View::Form => {
            view! {
                <div class="container mx-auto px-10">

                    <form id="challenge-evidence-submit-from" on:submit={submit_click}>

                        <div class="mb-5">
                            <label
                                for="profile-details"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Create a Positive Externality
                            </label>
                            <MarkdownField
                                set_markdown={set_markdown}
                                name={String::from("positive-externality-post")}
                                class={String::from(
                                    "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                )}
                            />

                        </div>

                        <button
                            type="submit"
                            id="challenge-evidence-submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Submit
                        </button>

                    </form>
                    <br />
                    <p class="text-gray-900 dark:text-white">
                        {move || pending().then(|| "Loading...")}
                    </p>
                    <p>{move || cid_value()}</p>
                </div>
            }.into_any()
        }

        View::Success => view! {
            <div>
                <SignTransaction post_cid={post_cid()} />
            </div>
        }.into_any(),
    }
    };

    view! {
        <div>
            <Nav />
            {move || render_view()}
        </div>
    }
}
