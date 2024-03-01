use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ChangePeriod(department_required_fund_id: u64) -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        navigate(
            &format!(
                "department-funding-change-period/{}",
                department_required_fund_id.clone()
            ),
            Default::default(),
        );
    };

    view! {
        <div>
            <form
                class="max-w-5xl mx-auto max-md:mx-10"
                id="apply-juror-submit-from"
                on:submit=submit_click
            >
                <button
                    type="submit"
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >

                    Change Period
                </button>
            </form>
        </div>
    }
}
