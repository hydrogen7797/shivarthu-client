use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use polkadot::runtime_types::pallet_schelling_game_shared::types::Period;
use polkadot::runtime_types::pallet_sortition_sum_game::types::SumTreeName;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data(user_to_calculate: String, set_period: WriteSignal<Option<Period>>) {
    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();

    let account_id32 = AccountId32::from_str(&user_to_calculate).unwrap();

    let validation_block_storage = polkadot::storage()
        .positive_externality()
        .validation_block(account_id32.clone());

    let validation_block = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&validation_block_storage)
        .await
        .unwrap();

    if validation_block.is_some() {
        let key = SumTreeName::PositiveExternality {
            user_address: account_id32.clone(),
            block_number: validation_block.unwrap(),
        };

        let period_storage = polkadot::storage().schelling_game_shared().period_name(key);
        let period = client
            .storage()
            .at_latest()
            .await
            .unwrap()
            .fetch(&period_storage)
            .await
            .unwrap();
        gloo::console::log!(format!("period in block: {:?}", period));
        set_period(period);
    }
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
