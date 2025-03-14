use crate::constants::constant::IROH_UPLOAD_SERVER;
use gloo::net::http::Request;
use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::{File, FormData};

#[derive(Deserialize)]
struct IrohResponse {
    ticket: String,
    node_id: String,
    blob_hash: String,
    blob_format: String,
}

pub async fn upload_file_iroh(file: File, name: String) -> String {
    let formdata = FormData::new().unwrap();
    formdata.append_with_blob(&name, &file).unwrap();

    let response = Request::post(IROH_UPLOAD_SERVER)
        .body(formdata)
        .unwrap()
        .send()
        .await
        .unwrap();

    let body = response.json::<IrohResponse>().await.unwrap();
    body.blob_hash
}

pub async fn upload_json_string_iroh(data: &str, name: String) -> String {
    let formdata = web_sys::FormData::new().unwrap();
    formdata.append_with_str(&name, data).unwrap();

    let response = Request::post(IROH_UPLOAD_SERVER)
        .body(formdata)
        .unwrap()
        .send()
        .await
        .unwrap();

    let body = response.json::<IrohResponse>().await.unwrap();
    body.blob_hash
}
