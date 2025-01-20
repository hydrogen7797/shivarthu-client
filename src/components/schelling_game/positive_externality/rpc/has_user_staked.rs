use crate::constants::constant::NODE_URL;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;

async fn load_data(user_to_calculate: String, check_account: String) -> bool {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    // gloo::console::log!(user_to_calculate.clone(), check_account.clone());
    let result: bool = client
        .request(
            "positiveexternality_has_user_staked",
            rpc_params![user_to_calculate, check_account],
        )
        .await
        .unwrap();
    result
}

#[component]
pub fn HasUserStaked(
    user_to_calculate: String,
    check_account: ReadSignal<String>,
) -> impl IntoView {
    let async_data =
        LocalResource::new(move || load_data(user_to_calculate.clone(), check_account().clone()));

    let async_result = move || {
        async_data
            .get()
            .as_deref()
            .map(|data| {
                if *data == false {
                    view! {
                        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                            <p>Value: {data.clone()}, you have not staked</p>
                        </div>
                    }
                    .into_any()
                } else {
                    view! {
                        <div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative" role="alert">
                            <p>Value: {data.clone()}, you have staked</p>
                        </div>
                    }
                    .into_any()
                }
            })
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
