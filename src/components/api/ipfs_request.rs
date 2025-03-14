use crate::components::api::blob_to_bytes::{blob_to_hash_string, file_to_hash_string};
use crate::components::api::select_ipfs_provider::IPFSProvider;
use crate::components::api::select_ipfs_provider::{CRUST_GATEWAY, WEB3_STORAGE_API_UPLOAD};
use crate::components::api::upload_file_everland::{upload_everland_file, upload_everland_string};
use crate::constants::auth::{CRUST_TOKEN, WEB3_STORAGE_TOKEN};

use gloo::net::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use web_sys::{Blob, File, FormData};

use crate::components::api::iroh::iroh_api::{upload_file_iroh, upload_json_string_iroh};
#[derive(Serialize, Deserialize)]
pub struct IPFSResponse {
    pub Name: String,
    pub Hash: String,
    pub Size: String,
}

#[derive(Serialize, Deserialize)]
pub struct CidResponse {
    pub cid: String,
}

pub async fn ipfs_call(ipfs_provider: IPFSProvider, file: File, name: String) -> String {
    match ipfs_provider {
        IPFSProvider::Crust => ipfs_call_crust(file, name).await,
        IPFSProvider::Web3Storage => ipfs_call_web3storage(file, name).await,
        IPFSProvider::Everland => ipfs_call_everland(file, name).await,
        IPFSProvider::Iroh => upload_file_iroh(file, name).await,
    }
}

pub async fn ipfs_call_json_string(
    ipfs_provider: IPFSProvider,
    data: &str,
    name: String,
) -> String {
    match ipfs_provider {
        IPFSProvider::Crust => ipfs_call_json_string_crust(data, name).await,
        IPFSProvider::Web3Storage => ipfs_call_json_string_web3storage(data, name).await,
        IPFSProvider::Everland => ipfs_call_json_string_everland(data, name).await,
        IPFSProvider::Iroh => upload_json_string_iroh(data, name).await,
    }
}

pub async fn ipfs_call_crust(file: File, name: String) -> String {
    let formdata = FormData::new().unwrap();
    let _ = formdata.append_with_blob(&name, &file);
    let headers = Headers::new();
    headers.append(&"Authorization", CRUST_TOKEN);

    let data = Request::post(&format!("{CRUST_GATEWAY}/api/v0/add?pin=true"))
        .headers(headers)
        .body(formdata)
        .unwrap()
        .send()
        .await
        .unwrap();
    //    log!(data.text().await.unwrap());
    let body = data.json::<IPFSResponse>().await.unwrap();
    body.Hash
    // {"Name":"pexels-pixabay-326055.jpg","Hash":"QmcTJaN8SqkKLNVjWeKSVSK8zVXSodhBqxVkZZcUb1isLp","Size":"782686"}
}

pub async fn ipfs_call_json_string_crust(data: &str, name: String) -> String {
    let formdata = FormData::new().unwrap();
    let _ = formdata.append_with_str(&name, &data);
    let headers = Headers::new();
    headers.append(&"Authorization", CRUST_TOKEN);

    let data = Request::post(&format!("{CRUST_GATEWAY}/api/v0/add?pin=true"))
        .headers(headers)
        .body(formdata)
        .unwrap()
        .send()
        .await
        .unwrap();
    //    log!(data.text().await.unwrap());
    let body = data.json::<IPFSResponse>().await.unwrap();
    body.Hash
    // {"Name":"pexels-pixabay-326055.jpg","Hash":"QmcTJaN8SqkKLNVjWeKSVSK8zVXSodhBqxVkZZcUb1isLp","Size":"782686"}
}

pub async fn ipfs_call_web3storage(file: File, _name: String) -> String {
    let headers = Headers::new();
    headers.append(&"accept", "application/octet-stream");
    headers.append(&"Authorization", WEB3_STORAGE_TOKEN);

    let data = Request::post(WEB3_STORAGE_API_UPLOAD)
        .headers(headers)
        .body(file)
        .unwrap()
        .send()
        .await
        .unwrap();
    //    log!(data.text().await.unwrap());
    let body = data.json::<CidResponse>().await.unwrap();
    body.cid
    // {"Name":"pexels-pixabay-326055.jpg","Hash":"QmcTJaN8SqkKLNVjWeKSVSK8zVXSodhBqxVkZZcUb1isLp","Size":"782686"}
}

pub async fn ipfs_call_everland(file: File, _name: String) -> String {
    let hash = file_to_hash_string(file.clone());
    let cid = upload_everland_file(file, hash).await;
    cid
}
pub async fn ipfs_call_json_string_web3storage(data: &str, _name: String) -> String {
    let json_jsvalue = JsValue::from_str(&data);
    let json_jsvalue_array = js_sys::Array::from_iter(std::iter::once(json_jsvalue));
    let json_blob_result = Blob::new_with_str_sequence(&json_jsvalue_array);
    let json_blob = json_blob_result.unwrap();
    let headers = Headers::new();
    headers.append(&"accept", "application/octet-stream");
    headers.append(&"Authorization", WEB3_STORAGE_TOKEN);

    let data = Request::post(WEB3_STORAGE_API_UPLOAD)
        .headers(headers)
        .body(json_blob)
        .unwrap()
        .send()
        .await
        .unwrap();
    // log!(data.text().await.unwrap());

    let body = data.json::<CidResponse>().await.unwrap();
    body.cid
}

pub async fn ipfs_call_json_string_everland(data: &str, _name: String) -> String {
    let json_jsvalue = JsValue::from_str(&data);
    let json_jsvalue_array = js_sys::Array::from_iter(std::iter::once(json_jsvalue));
    let json_blob_result = Blob::new_with_str_sequence(&json_jsvalue_array);
    let json_blob = json_blob_result.unwrap();
    let hash = blob_to_hash_string(json_blob.clone());
    let cid = upload_everland_string(json_blob, hash).await;
    cid
}
