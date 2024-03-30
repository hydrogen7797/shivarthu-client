use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use polkadot::runtime_types::schelling_game_shared::types::Period;
use polkadot::runtime_types::sortition_sum_game::types::SumTreeName;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data(user_to_calculate: String, set_period: WriteSignal<Option<Period>>) {
    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();
}

pub fn get_period_fn(user_to_calculate: String) -> ReadSignal<Option<Period>> {
    let (period, set_period) = create_signal::<Option<Period>>(None);

    let action = create_action(
        |(user_to_calculate, set_period): &(String, WriteSignal<Option<Period>>)| {
            let user_to_calculate = user_to_calculate.clone();
            let set_period = set_period.clone();
            async move { load_data(user_to_calculate, set_period).await }
        },
    );

    action.dispatch((user_to_calculate.clone(), set_period));

    period
}
