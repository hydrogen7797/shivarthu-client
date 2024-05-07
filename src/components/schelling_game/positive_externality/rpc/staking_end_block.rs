use crate::constants::constant::NODE_URL;
use icondata;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data(user_to_calculate: String, set_end_period: WriteSignal<Option<u32>>) {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let result: Option<u32> = client
        .request(
            "positiveexternality_stakingperiodendblock",
            rpc_params![user_to_calculate],
        )
        .await
        .unwrap();
    set_end_period(result);
}

#[component]
pub fn StakingEndBlock(user_to_calculate: String) -> impl IntoView {
    let (end_period, set_end_period) = create_signal::<Option<u32>>(None);

    let action = create_action(
        |(user_to_calculate, set_end_period): &(String, WriteSignal<Option<u32>>)| {
            let user_to_calculate = user_to_calculate.clone();
            let set_end_period = set_end_period.clone();
            async move { load_data(user_to_calculate, set_end_period).await }
        },
    );

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch((user_to_calculate.clone(), set_end_period));
        },
        5000,
    );

    view! {
        <div>
            {move || {
                if end_period().is_some() {
                    view! {
                        <div>
                            {"Staking Period ends: "}
                            <span id="end-period-time">{move || end_period()}</span>
                        </div>
                    }
                } else {
                    view! {
                        <div>
                            {"Staking Period ends: "} <span id="end-period-time">
                                <Icon
                                    icon=icondata::ImSpinner6
                                    style="color: green"
                                    class="inline-block"
                                />
                            </span>
                        </div>
                    }
                }
            }}

        </div>
    }
}
