use crate::components::accounts::accounts_store::AccountStore;
use codee::string::JsonSerdeCodec;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_use::storage::use_local_storage;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum PhraseFromPassError {
    #[error("Password is empty")]
    PasswordEmpty,
    #[error("Error parsing to number: `{0}`")]
    ParseIntError(String),
}

#[component]
pub fn SetPhraseFromPass() -> impl IntoView {
    let (password, set_password) = create_signal(Ok("".to_string()));
    let (account_store, _set_account_store, _reset_account_store) =
        use_local_storage::<AccountStore, JsonSerdeCodec>("account-store-state");

    let set_password_input = move |ev| {
        let password_string = event_target_value(&ev);
        gloo::console::log!(password_string.clone());
        let result = if password_string.is_empty() {
            gloo::console::log!("Strin is empty");

            Err(PhraseFromPassError::PasswordEmpty)
        } else {
            Ok(password_string)
        };
        set_password(result);
    };
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        gloo::console::log!(format!("{:?}", account_store()));
        if let Some(hash) = account_store().hash {
            let mc = new_magic_crypt!(password().unwrap(), 256);
            let _seed = mc.decrypt_base64_to_string(&hash).unwrap();
        }
    };
    view! {
        <>
            <div>
                <form class="max-w-sm mx-auto" id="seed-submit-form" on:submit={submit_click}>
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
                            prop:value={move || password().unwrap()}
                            on:input={set_password_input}
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
