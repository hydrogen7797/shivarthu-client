use crate::components::schelling_game::department_funding::get_department_funding_sign_in::SignTransaction;
use crate::services::common_imp::View;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn GetDepartmentFunding(department_required_fund_id: u64) -> impl IntoView {
    // gloo::console::log!(department_required_fund_id());
    let (current_view, set_current_view) = create_signal(View::Form);
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        set_current_view(View::Success);
    };

    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div class="container mx-auto px-10">
                    <form id="get-incentives-submit-from" on:submit=submit_click>
                        <button
                            type="submit"
                            id="get-incentives-submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >
                            Get Incentives
                        </button>
                    </form>
                </div>
            }
        }
        View::Success => {
            view! {
                <div>
                    <SignTransaction department_required_fund_id=department_required_fund_id
                        .clone()/>

                </div>
            }
        }
    };

    view! { <div>{move || render_view()}</div> }
}
