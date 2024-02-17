use crate::components::api::ipfs_fetch_response::ipfs_fetch_response;
use crate::constants::constant::IPFSFetchProvider;
use gloo::console::log;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileFetchResponse {
    pub version: String,
    pub name: String,
    pub details: String,
    pub profile_video_cid: String,
}

pub async fn ipfs_fetch(
    hash: &str,
    ipfs_fetch_provider: IPFSFetchProvider<'_>,
) -> ProfileFetchResponse {
    let resp = ipfs_fetch_response(hash, &ipfs_fetch_provider.address).await;
    let body = resp.json::<ProfileFetchResponse>().await.unwrap();
    // log!(body.name);
    body
}
