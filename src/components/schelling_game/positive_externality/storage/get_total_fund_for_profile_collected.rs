use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::*;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

#[component]
pub fn TotalFundProfileCollected(user_to_calculate: String) -> impl IntoView {
    let async_load = create_resource(
        move || user_to_calculate.clone(),
        |user_to_calculate| async move {
            let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                .await
                .unwrap();
            let account_id32 = AccountId32::from_str(&user_to_calculate).unwrap();
            let total_profile_fund_collected = polkadot::storage()
                .positive_externality()
                .profile_total_fund_collected(account_id32);

            let fund_collected_value = client
                .storage()
                .at_latest()
                .await
                .unwrap()
                .fetch_or_default(&total_profile_fund_collected)
                .await
                .unwrap();

            let registration_fee_storage =
                polkadot::storage().positive_externality().registration_fee();
            let registration_fee_value = client
                .storage()
                .at_latest()
                .await
                .unwrap()
                .fetch_or_default(&registration_fee_storage)
                .await
                .unwrap();

            let fund_needed = registration_fee_value
                .checked_sub(fund_collected_value)
                .unwrap();
            (fund_collected_value, registration_fee_value, fund_needed)
        },
    );

    view! {
        <div>
            {move || match async_load.get() {
                None => {
                    view! {
                        <p>
                            <span class="loading loading-spinner text-primary"></span>
                            Loading...
                        </p>
                    }
                        .into_view()
                }
                Some(data) => {
                    view! {
                        <div>Total fund collected: {data.0}</div>
                        <div>Registration fee: {data.1}</div>
                        <div>Fund needed : {data.2}</div>
                    }
                        .into_view()
                }
            }}

        </div>
    }
}
