use crate::components::accounts::accounts_store::AccountStore;
use crate::components::navigation::nav::Nav;
use crate::js_extension_binding;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn AddAccount() -> impl IntoView {
    let (account_store, set_account_store, reset_account_store) =
        use_local_storage::<AccountStore, JsonCodec>("account-store-state");
    let (seed, set_seed) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        let account = js_extension_binding::get_account_address_from_seed(seed());
        let mc = new_magic_crypt!(password(), 256);
        let base64 = mc.encrypt_str_to_base64(seed());

        set_account_store.update(|acct| {
            acct.hash = Some(base64);
            acct.account_address = Some(account);
        })
    };
    view! {
        <>
            <Nav/>
            <div>
                <form class="max-w-sm mx-auto" d="seed-submit-from" on:submit={submit_click}>
                    <div class="mb-5">
                        <label
                            for="seed"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            Seed
                        </label>
                        <input
                            type="seed"
                            id="seed"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            placeholder="Enter the seed"
                            required
                            prop:value={move || seed()}
                            on:input={move |e| set_seed(event_target_value(&e))}
                        />
                    </div>
                    <div class="mb-5">
                        <label
                            for="password"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            Your password
                        </label>
                        <input
                            type="password"
                            id="password"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            required
                            prop:value={move || password()}
                            on:input={move |e| set_password(event_target_value(&e))}
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
        </>
    }
}
