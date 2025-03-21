use crate::constants::constant::NODE_URL;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;
use leptos::task::spawn_local;

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
    let (data, set_data) = signal(None::<bool>);

    Effect::new(move |_| {
        let department_required_fund_id = department_required_fund_id.clone();
        let account = check_account.get(); // Reactive dependency on `check_account`

        // Spawn an asynchronous task to fetch data
        spawn_local(async move {
            let result = load_data(department_required_fund_id, account).await;
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
