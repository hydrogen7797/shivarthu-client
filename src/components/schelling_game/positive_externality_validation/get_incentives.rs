use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::positive_externality_validation::get_incentives_sign_in::SignTransaction;
use crate::services::common_imp::View;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;

#[component]
pub fn GetIncentives() -> impl IntoView {
    let params = use_params_map();

    let user_to_calculate =
        move || params.with(|params| params.get("user_to_calculate").cloned().unwrap_or_default());

    // gloo::console::log!(user_to_calculate());
    let (current_view, set_current_view) = create_signal(View::Form);
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        set_current_view(View::Success);
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
                        <button
                            type="submit"
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
                    <SignTransaction user_to_calculate=user_to_calculate()/>

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
