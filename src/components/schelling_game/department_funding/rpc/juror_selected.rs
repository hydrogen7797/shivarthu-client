use crate::constants::constant::NODE_URL;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::*;

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
    let async_data = create_resource(
        move || (department_required_fund_id.clone(), check_account().clone()),
        |(department_required_fund_id, check_account)| async move {
            load_data(department_required_fund_id, check_account).await
        },
    );
    view! {
        <div>
            {move || match async_data.get() {
                None => {
                    view! {
                        <p>
                            <span class="loading loading-dots loading-xs"></span>
                        </p>
                    }
                        .into_view()
                }
                Some(data) => {
                    if data == false {
                        view! {
                            <div role="alert" class="alert alert-error">
                                <p>Value: {data} , you are not selected as juror</p>
                            </div>
                        }
                            .into_view()
                    } else {
                        view! {
                            <div role="alert" class="alert alert-success">
                                <p>Value: {data} , you are selected as juror</p>
                            </div>
                        }
                            .into_view()
                    }
                }
            }}

        </div>
    }
}
