use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data(department_required_fund_id: u64, set_challenger_fee: WriteSignal<Option<u128>>) {
    let account_id32 = AccountId32::from_str(&department_required_fund_id).unwrap();

    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();
    let challenger_fee_storage = polkadot::storage()
        .department_funding()
        .registration_challenge_fee();

    let challenger_fee_value = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch_or_default(&challenger_fee_storage)
        .await
        .unwrap();
    set_challenger_fee(Some(challenger_fee_value));
}

#[component]
pub fn ChallengerFees(department_required_fund_id: u64) -> impl IntoView {
    let (challenger_fee, set_challenger_fee) = create_signal::<Option<u128>>(None);

    let action = create_action(
        |(department_required_fund_id, set_challenger_fee): &(u64, WriteSignal<Option<u128>>)| {
            let department_required_fund_id = department_required_fund_id.clone();
            let set_challenger_fee = set_challenger_fee.clone();
            async move { load_data(department_required_fund_id, set_challenger_fee).await }
        },
    );

    create_effect(move |_| {
        action.dispatch((department_required_fund_id.clone(), set_challenger_fee));
    });

    view! { <div>{move || format!("{:#?}", challenger_fee())}</div> }
}
