use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

async fn load_data(user_to_calculate: String, set_challenger_fee: WriteSignal<Option<u128>>) {
    let account_id32 = AccountId32::from_str(&user_to_calculate).unwrap();

    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();
    let challenger_fee_storage = polkadot::storage()
        .positive_externality()
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
pub fn ChallengerFees(user_to_calculate: String) -> impl IntoView {
    let (challenger_fee, set_challenger_fee) = create_signal::<Option<u128>>(None);

    let action = create_action(
        |(user_to_calculate, set_challenger_fee): &(String, WriteSignal<Option<u128>>)| {
            let user_to_calculate = user_to_calculate.clone();
            let set_challenger_fee = set_challenger_fee.clone();
            async move { load_data(user_to_calculate, set_challenger_fee).await }
        },
    );

    create_effect(move |_| {
        action.dispatch((user_to_calculate.clone(), set_challenger_fee));
    });

    view! { <div>{move || format!("{:#?}", challenger_fee())}</div> }
}
