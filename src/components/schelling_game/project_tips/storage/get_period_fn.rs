use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use polkadot::runtime_types::schelling_game_shared::types::Period;
use polkadot::runtime_types::sortition_sum_game::types::SumTreeName;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data(project_id: u64, set_period: WriteSignal<Option<Period>>) {
    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();
}

pub fn get_period_fn(project_id: u64) -> ReadSignal<Option<Period>> {
    let (period, set_period) = create_signal::<Option<Period>>(None);

    let action = create_action(
        |(project_id, set_period): &(u64, WriteSignal<Option<Period>>)| {
            let project_id = project_id.clone();
            let set_period = set_period.clone();
            async move { load_data(project_id, set_period).await }
        },
    );

    action.dispatch((project_id.clone(), set_period));

    period
}
