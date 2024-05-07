use crate::constants::constant::NODE_URL;
use icondata;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data(
    user_to_calculate: String,
    set_drawing_period: WriteSignal<Option<(u64, u64, bool)>>,
) {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let result: (u64, u64, bool) = client
        .request(
            "positiveexternality_drawingperiodend",
            rpc_params![user_to_calculate],
        )
        .await
        .unwrap();
    set_drawing_period(Some(result));
}

#[component]
pub fn DrawingEndBlock(user_to_calculate: String) -> impl IntoView {
    let (drawing_period, set_drawing_period) = create_signal::<Option<(u64, u64, bool)>>(None);

    let action = create_action(
        |(user_to_calculate, set_drawing_period): &(
            String,
            WriteSignal<Option<(u64, u64, bool)>>,
        )| {
            let user_to_calculate = user_to_calculate.clone();
            let set_drawing_period = set_drawing_period.clone();
            async move { load_data(user_to_calculate, set_drawing_period).await }
        },
    );

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch((user_to_calculate.clone(), set_drawing_period));
        },
        5000,
    );

    view! {
        <div>
            {move || {
                if drawing_period().is_some() {
                    view! {
                        <div>
                            {"Drawing Period ends: "}
                            <span id="end-period-time">{move || drawing_period().unwrap().2}</span>
                        </div>
                    }
                } else {
                    view! {
                        <div>
                            {"Drawing Period ends: "} <span id="end-period-time">
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
