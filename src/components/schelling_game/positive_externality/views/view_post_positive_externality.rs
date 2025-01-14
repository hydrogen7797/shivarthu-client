use crate::components::api::ipfs_fetch_response::ipfs_fetch_response;
use crate::constants::constant::IPFSFetchProvider;
use crate::constants::constant::DEFAULT_IPFS_FETCH_PROVIDER;
use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use leptos::task::spawn_local;
use polkadot::runtime_types::pallet_support::Content;
use serde::{Deserialize, Serialize};
use subxt::{OnlineClient, PolkadotConfig};

#[component]
pub fn ViewPostPositiveExternality(id: u64) -> impl IntoView {
    let (data_post, set_data_post) = signal("".to_string());

    Effect::new(move |_| {
        spawn_local(async move {
            let data = get_post_data(id).await;

            gloo::console::log!(data.clone(), "In data");

            set_data_post(data);

        });
    });
    view! { <div>{data_post}</div> }
}

async fn get_post_data(id: u64) -> String {
    let client = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
        .await
        .unwrap();

    let post_storage = polkadot::storage().positive_externality().post_by_id(id);

    let value = client
        .storage()
        .at_latest()
        .await
        .unwrap()
        .fetch(&post_storage)
        .await
        .unwrap();

    let post = value.unwrap();

    let content = post.content;
    let mut resp_option: Option<PositiveExternalityResponse> = None;

    if let Content::IPFS(ipfsdata) = content {
        let ipfs_hash = String::from_utf8(ipfsdata).unwrap();
        gloo::console::log!("ipfs_hash", ipfs_hash.clone());

        let resp = ipfs_fetch(&ipfs_hash, DEFAULT_IPFS_FETCH_PROVIDER).await;
        resp_option = Some(resp.clone());
    }

    let text = resp_option.unwrap().details;
    
    text
    // "hello2".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositiveExternalityResponse {
    pub version: String,
    pub details: String,
}

pub async fn ipfs_fetch(
    hash: &str,
    ipfs_fetch_provider: IPFSFetchProvider<'_>,
) -> PositiveExternalityResponse {
    let resp = ipfs_fetch_response(hash, &ipfs_fetch_provider.address).await;
    let body = resp.json::<PositiveExternalityResponse>().await.unwrap();
    // log!(body.name);
    body
}
