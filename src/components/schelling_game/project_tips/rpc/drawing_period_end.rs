use crate::constants::constant::NODE_URL;
use icondata;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data(project_id: u64, set_drawing_period: WriteSignal<Option<(u64, u64, bool)>>) {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let result: (u64, u64, bool) = client
        .request("projecttips_drawingperiodend", rpc_params![project_id])
        .await
        .unwrap();
    set_drawing_period(Some(result));
}

#[component]
pub fn DrawingEndBlock(project_id: u64) -> impl IntoView {
    let (drawing_period, set_drawing_period) = create_signal::<Option<(u64, u64, bool)>>(None);

    let action = create_action(
        |(project_id, set_drawing_period): &(u64, WriteSignal<Option<(u64, u64, bool)>>)| {
            let project_id = project_id.clone();
            let set_drawing_period = set_drawing_period.clone();
            async move { load_data(project_id, set_drawing_period).await }
        },
    );

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch((project_id.clone(), set_drawing_period));
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
