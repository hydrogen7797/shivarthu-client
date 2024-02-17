use crate::components::navigation::nav::Nav;
use crate::services::common_imp::View;
use leptos::ev::SubmitEvent;
use leptos::*;

#[component]
pub fn ApplyJurors(profile_user_account: String) -> impl IntoView {
    let (current_view, set_current_view) = create_signal(View::Form);
    let (juror_stake, set_juror_stake) = create_signal::<Option<u32>>(None);

    let submit_action = create_action(|input: &()| async { todo!() });

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        submit_action.dispatch(());

        set_current_view(View::Success);
    };

    let stake_value = move |value: String| {
        let stake = value.parse::<u32>().expect("Invalid input");
        set_juror_stake(Some(stake));
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
                                for="profile-name"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Your Name
                            </label>
                            <input
                                type="text"
                                id="profile-name"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                prop:value=move || juror_stake()
                                on:input=move |e| stake_value(event_target_value(&e))
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
            view! { <div></div> }
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
