use crate::constants::constant::NODE_URL;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;

async fn load_data(department_required_fund_id: u64, check_account: String) -> bool {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    // gloo::console::log!(department_required_fund_id.clone(), check_account.clone());
    let result: bool = client
        .request(
            "departmentfunding_selectedjuror",
            rpc_params![department_required_fund_id, check_account],
        )
        .await
        .unwrap();
    result
}

#[component]
pub fn JurorSelected(
    department_required_fund_id: u64,
    check_account: ReadSignal<String>,
) -> impl IntoView {
    let async_data =LocalResource::new(
        move || load_data(department_required_fund_id.clone(), check_account().clone()),
       
    );

    let async_result = move || {
        async_data
            .get()
            .as_deref()
            .map(|data| {
                if *data == false {
                    view! {
                        <div role="alert" class="alert alert-error">
                            <p>Value: {data.clone()} , you are not selected as juror</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div role="alert" class="alert alert-success">
                            <p>Value: {data.clone()} , you are selected as juror</p>
                        </div>
                    }.into_any()
                }
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| view! {
                <div class="alert">
                    <span class="loading loading-spinner"></span>
                    "Loading... Please sign with extension."
                </div>
            }
            .into_any())
    };
    view! {
        <div>{async_result}
        </div>
    }
}
