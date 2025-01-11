use crate::constants::constant::NODE_URL;
use icondata;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data(
    profile_user_account: String,
    set_drawing_period: WriteSignal<Option<(u64, u64, bool)>>,
) {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let result: (u64, u64, bool) = client
        .request(
            "profilevalidation_drawingperiodend",
            rpc_params![profile_user_account],
        )
        .await
        .unwrap();
    set_drawing_period(Some(result));
}

#[component]
pub fn DrawingEndBlock(profile_user_account: String) -> impl IntoView {
    let (drawing_period, set_drawing_period) = signal::<Option<(u64, u64, bool)>>(None);

    let action: Action<(String, WriteSignal<Option<(u64, u64, bool)>>), (), LocalStorage> = Action::new_unsync(
        |(profile_user_account, set_drawing_period): &(
            String,
            WriteSignal<Option<(u64, u64, bool)>>,
        )| {
            let profile_user_account = profile_user_account.clone();
            let set_drawing_period = set_drawing_period.clone();
            async move { load_data(profile_user_account, set_drawing_period).await }
        },
    );

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch((profile_user_account.clone(), set_drawing_period));
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
                        .into_any()
                } else {
                    view! {
                        <div>
                            {"Drawing Period ends: "} <span id="end-period-time">
                                <Icon icon=icondata::ImSpinner6 style="color: green"/>
                            </span>
                        </div>
                    }
                        .into_any()
                }
            }}

        </div>
    }
}
