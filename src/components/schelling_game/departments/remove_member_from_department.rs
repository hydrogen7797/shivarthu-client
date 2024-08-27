use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::departments::remove_member_from_department_sign_in::SignTransaction;
use crate::services::common_imp::View;
use leptos::ev::SubmitEvent;
use leptos::*;

#[component]
pub fn RemoveMemberFromDepartment(department_id: u64) -> impl IntoView {
    // gloo::console::log!(user_to_calculate());
    let (current_view, set_current_view) = create_signal(View::Form);

    let (account_id, set_account_id) = create_signal(String::from(""));
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        set_current_view(View::Success);
    };

    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div class="container mx-auto px-10">
                    <form

                        id="add-department-member-submit-from"
                        on:submit=submit_click
                    >
                        <div class="mb-5">
                            <label
                                for="Department Member"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                "Department Member"
                            </label>
                            <input
                                type="text"
                                id="department-member"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |ev| set_account_id(event_target_value(&ev))
                            />
                        </div>
                        <button
                            type="submit"
                            id="commit-vote-submit"
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
                    <SignTransaction account_id=account_id() department_id=department_id.clone()/>

                </div>
            }
        }
    };

    view! {
        <div>
            <Nav/>
            {move || render_view()}
        </div>
    }
}
