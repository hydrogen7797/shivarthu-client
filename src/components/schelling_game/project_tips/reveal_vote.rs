use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::project_tips::reveal_vote_sign_in::SignTransaction;
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn CommitVote(project_id: u64) -> impl IntoView {
    // gloo::console::log!(project_id());
    let (current_view, set_current_view) = create_signal(View::Form);
    let (choice, set_choice) = create_signal::<Result<Option<u128>, ErrorString>>(Ok(None));
    let (salt, set_salt) = create_signal(String::from(""));
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        if choice().unwrap().is_some() {
            set_current_view(View::Success);
        } else {
            panic!("Choice not set");
        }
    };

    let choice_changed = move |value: String| {
        let choice_value = value.parse::<u128>().expect("Invalid input");
        gloo::console::log!(choice_value);

        set_choice(Ok(Some(choice_value)));
    };

    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div>
                    <form
                        class="max-w-5xl mx-auto max-md:mx-10"
                        id="apply-juror-submit-from"
                        on:submit=submit_click
                    >

                        <div class="mb-5">
                            <label
                                for="choice"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Choice
                            </label>
                            <input
                                type="number"
                                id="choice"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |ev| choice_changed(event_target_value(&ev))
                            />
                        </div>
                        <div class="mb-5">
                            <label
                                for="salt"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Salt
                            </label>
                            <input
                                type="text"
                                id="salt"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |ev| set_salt(event_target_value(&ev))
                            />
                        </div>
                        <button
                            type="submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Submit
                        </button>
                    </form>
                </div>
            }
        }
        View::Success => {
            view! {
                <div>
                    <SignTransaction
                        salt=salt()
                        choice=choice().unwrap().unwrap()
                        project_id=project_id.clone()
                    />

                </div>
            }
        }
        _ => {
            view! { <div></div> }
        }
    };

    view! {
        <>
            <Nav/>
            {move || render_view()}
        </>
    }
}
