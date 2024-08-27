use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn ChangePeriod(profile_user_account: String) -> impl IntoView {
    let navigate = leptos_router::use_navigate();

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        navigate(
            &format!(
                "profile-validation-change-period/{}",
                profile_user_account.clone()
            ),
            Default::default(),
        );
    };

    view! {
        <div class="container mx-auto px-10">
            <form

                id="change-period-submit-from"
                on:submit=submit_click
            >
                <button
                    type="submit"
                    id="change-period-submit"
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >

                    Change Period
                </button>
            </form>
        </div>
    }
}
