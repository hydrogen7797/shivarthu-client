use crate::components::balance::transfer_balance_sign_in::SignTransaction;
use crate::components::navigation::nav::Nav;
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn TransferBalance() -> impl IntoView {
    let (current_view, set_current_view) = signal(View::Form);

    let (dest_account, set_dest_account) = signal(String::from(""));
    let (transfer_balance, set_transfer_balance) =
        signal::<Result<u128, ErrorString>>(Ok(0));

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        set_current_view(View::Success);
    };

    let transfer_value_fn = move |value: String| {
        let transfer_value = value.parse::<u128>().expect("Invalid input");
        gloo::console::log!(transfer_value);

        set_transfer_balance(Ok(transfer_value));
    };

    let render_view = move || match current_view() {
        View::Form => {
            view! {
                <div>
                    <form
                        class="container mx-auto px-10"
                        id="apply-juror-submit-from"
                        on:submit=submit_click
                    >
                        <div class="mb-5">
                            <label
                                for="destination account"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Destination Account
                            </label>
                            <input
                                type="text"
                                id="destination account"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |e| set_dest_account(event_target_value(&e))
                            />
                        </div>

                        <div class="mb-5">
                            <label
                                for="transfer-balance"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Transfer Balance
                            </label>
                            <input
                                type="number"
                                id="transfer-balance"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |e| transfer_value_fn(event_target_value(&e))
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
            }.into_any()
        }
        View::Success => {
            view! {
                <div>
                    <SignTransaction
                        dest_account=dest_account()
                        transfer_balance=transfer_balance().unwrap()
                    />

                </div>
            }.into_any()
        }
        _ => {
            view! { <div></div> }.into_any()
        }
    };

    view! {
        <>
            <Nav/>
            {move || render_view()}
        </>
    }
}
