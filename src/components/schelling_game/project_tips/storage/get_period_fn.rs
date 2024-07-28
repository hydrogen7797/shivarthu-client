use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use polkadot::runtime_types::pallet_schelling_game_shared::types::Period;
use polkadot::runtime_types::pallet_sortition_sum_game::types::SumTreeName;
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data(project_id: u64, set_period: WriteSignal<Option<Period>>) {
    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();

    let validation_block_storage = polkadot::storage()
        .project_tips()
        .validation_block(project_id);

    let validation_block = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&validation_block_storage)
        .await
        .unwrap();

    if validation_block.is_some() {
        let key = SumTreeName::ProjectTips {
            project_id: project_id,
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
