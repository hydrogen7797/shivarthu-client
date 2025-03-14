use crate::components::login::get_login_account::AccountState;
use crate::constants::constant::NODE_URL;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;

async fn load_data(user_to_calculate: String, check_account: String) -> u64 {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    // gloo::console::log!(user_to_calculate.clone(), check_account.clone());
    let result: u64 = client
        .request(
            "positiveexternality_user_staked_value",
            rpc_params![user_to_calculate, check_account],
        )
        .await
        .unwrap();
    result
}

#[component]
pub fn UserStakedValue(
    user_to_calculate: String,
    account_state: Signal<AccountState>,
) -> impl IntoView {
    let async_data = LocalResource::new(move || {
        load_data(
            user_to_calculate.clone(),
            account_state().account_id.clone(),
        )
    });

    let async_result = move || {
        async_data
            .get()
            .as_deref()
            .map(|data| {
                    view! {
                        <div
                            class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded"
                            role="alert"
                        >
                            <p>Your staked amount: {data.clone()}</p>
                        </div>
                    }
                    .into_any()
                }
            )
            // This loading state will only show before the first load
            .unwrap_or_else(|| {
                view! {
                    <div class="alert">
                        <span class="loading loading-spinner"></span>
                        "Loading... Please sign with extension."
                    </div>
                }
                .into_any()
            })
    };
    view! { <div>{async_result}</div> }
}
