use crate::constants::constant::NODE_URL;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;
use leptos::task::spawn_local;

async fn load_data(profile_user_account: String, check_account: String) -> bool {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    // gloo::console::log!(profile_user_account.clone(), check_account.clone());
    let result: bool = client
        .request(
            "profilevalidation_selectedjuror",
            rpc_params![profile_user_account, check_account],
        )
        .await
        .unwrap();
    result
}

#[component]
pub fn JurorSelected(
    profile_user_account: String,
    check_account: ReadSignal<String>,
) -> impl IntoView {
    let (data, set_data) = signal(None::<bool>);

    Effect::new(move |_| {
        let profile_user_account = profile_user_account.clone();
        let account = check_account.get(); // Reactive dependency on `check_account`

        // Spawn an asynchronous task to fetch data
        spawn_local(async move {
            let result = load_data(profile_user_account, account).await;
            set_data.set(Some(result)); // Update the signal with the fetched data
        });
    });

    // Define the reactive view based on the current state of `data`
    let async_result = move || {
        data.get().as_ref().map_or_else(
            || {
                // Loading state
                view! { <div></div> }
                .into_any()
            },
            |data| {
                if *data == false {
                    view! {
                        <div
                            role="alert"
                            class="flex items-center gap-3 p-4 border-l-4 border-red-500 bg-red-100 text-red-800 rounded-xl shadow-md"
                        >
                            <p>Value: {data.to_string()}, you are not selected as juror</p>
                        </div>
                    }
                    .into_any()
                } else {
                    view! {
                        <div
                            role="alert"
                            class="flex items-center gap-3 p-4 border-l-4 border-green-500 bg-green-100 text-green-800 rounded-xl shadow-md"
                        >
                            <p>Value: {data.to_string()}, you are selected as juror</p>
                        </div>
                    }
                    .into_any()
                }
            },
        )
    };
    view! {
        <div>{async_result}
        </div>
    }
}
