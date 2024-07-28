use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use polkadot::runtime_types::pallet_schelling_game_shared::types::Period;
use polkadot::runtime_types::pallet_sortition_sum_game::types::SumTreeName;
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data(department_required_fund_id: u64, set_period: WriteSignal<Option<Period>>) {
    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();

    let validation_block_storage = polkadot::storage()
        .department_funding()
        .validation_block(department_required_fund_id);

    let validation_block = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&validation_block_storage)
        .await
        .unwrap();

    if validation_block.is_some() {
        let key = SumTreeName::DepartmentRequiredFund {
            department_required_fund_id: department_required_fund_id,
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

pub fn get_period_fn(department_required_fund_id: u64) -> ReadSignal<Option<Period>> {
    let (period, set_period) = create_signal::<Option<Period>>(None);

    let action = create_action(
        |(department_required_fund_id, set_period): &(u64, WriteSignal<Option<Period>>)| {
            let department_required_fund_id = department_required_fund_id.clone();
            let set_period = set_period.clone();
            async move { load_data(department_required_fund_id, set_period).await }
        },
    );

    action.dispatch((department_required_fund_id.clone(), set_period));

    period
}
