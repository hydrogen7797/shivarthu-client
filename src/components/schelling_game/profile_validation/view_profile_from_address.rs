use crate::components::schelling_game::profile_validation::fetch_ipfs_profile::{
    ipfs_fetch, ProfileFetchResponse,
};
use crate::constants::constant::DEFAULT_IPFS_FETCH_PROVIDER;
use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use polkadot::runtime_types::pallet_support::Content;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::PolkadotConfig;


async fn transaction(
    profile_user_account: String   
) -> ProfileFetchResponse {

    let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
    .await
    .unwrap();
let account_id32 = AccountId32::from_str(&profile_user_account).unwrap();

let citizen_profile_storage = polkadot::storage()
    .profile_validation()
    .citizen_profile(account_id32);
let citizen_details = client
    .storage()
    .at_latest()
    .await
    .unwrap()
    .fetch(&citizen_profile_storage)
    .await
    .unwrap()
    .unwrap();

let content = citizen_details.content;

let mut resp_option: Option<ProfileFetchResponse> = None;

if let Content::IPFS(ipfsdata) = content {
    let ipfs_hash = String::from_utf8(ipfsdata).unwrap();
    gloo::console::log!("ipfs_hash", ipfs_hash.clone());

    let resp = ipfs_fetch(&ipfs_hash, DEFAULT_IPFS_FETCH_PROVIDER).await;
    resp_option = Some(resp.clone());
}

resp_option.unwrap()

}
#[component]
pub fn ViewProfileFromAddress() -> impl IntoView {
    let params = use_params_map();
    let profile_user_account = move || {
        params.with(|params| {
            params
                .get("profile_user_account")
                .unwrap_or_default()
        })
    };



    let async_load = LocalResource::new(
        move || transaction(profile_user_account())
       
    );


    let async_result = move || {
        async_load
            .get()
            .as_deref()
            .map(|data| view! {
                <div class="container mx-auto px-10">
                    <div class="mb-5">
                        <div class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                            <h2 class="heading">{"Name"}</h2>
                            <p class="data">{format!("{}", data.name.clone())}</p>
                        </div>

                    </div>
                    <div class="mb-5">
                        <div class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                            <h2 class="heading">{"Details"}</h2>
                            <p inner_html=data.details.clone()></p>
                        </div>

                    </div>
                    <div class="mb-5">
                        <div class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
                            <video width="320" height="240" controls=true>
                                <source
                                    src=format!(
                                        "{}{}",
                                        DEFAULT_IPFS_FETCH_PROVIDER.address,
                                        data.profile_video_cid.clone(),
                                    )

                                    type="video/mp4"
                                />
                                {"Your browser does not support the video tag."}
                            </video>
                        </div>

                    </div>
                </div>
            }.into_any())
            .unwrap_or_else(|| view! {
                <p>
                    <span class="loading loading-spinner text-primary"></span>
                    Loading...
                </p>
            }
            .into_any())
    };

    // create_effect(move |_| async_load.get());

    view! { <div>{async_result}</div> }
}
