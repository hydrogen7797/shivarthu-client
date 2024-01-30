use wasm_bindgen::prelude::*;
use js_sys::Promise;
use web_sys::{File, Blob};


#[wasm_bindgen(module = "/src/package.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn helloworld();

    #[wasm_bindgen]
    pub fn get_account_address_from_seed(mnemonic: String) -> String;

    #[wasm_bindgen]
    pub fn transfer_balance(wsprovider: String, mnemonic: String, credit_user: String, value: u32) -> Promise;

    #[wasm_bindgen]
    pub fn add_profile(wsprovider: String, mnemonic: String, ipfs_string: String) -> Promise;

    #[wasm_bindgen]
    pub fn add_profile_stake(wsprovider: String, mnemonic: String, profile_user_account: String, amount_to_fund: u32) -> Promise;

    #[wasm_bindgen]
    pub fn chat_text(token: String, prompt: String, model: String) -> Promise;

    #[wasm_bindgen]
    pub fn challege_profile(wsprovider: String, mnemonic: String, profile_user_account: String, ipfs_string: String) -> Promise;

    #[wasm_bindgen(js_name = getAccounts)]
    pub fn js_get_accounts() -> Promise;
    
    #[wasm_bindgen(js_name = signPayload)]
    pub fn js_sign_payload(payload: String, source: String, address: String) -> Promise;
   
    // bafykbzacecb53zxvj6q6q63rintnpcuhufbojz56q3zg4xxpzmzgmi2pplnmi

    #[wasm_bindgen(js_name = getSTSCredentials)]
    pub fn get_sts_credentials(key: String, secret: String) -> Promise;

    #[wasm_bindgen(js_name = uploadFile4everland)]
    pub fn upload_file_4everland(endpoint: String, bucket: String, folder: String, data: String, file_name: String, key: String, secret: String, session_token: String) -> Promise;

    #[wasm_bindgen(js_name = getCid)]
    pub fn get_cid(string_data: String) -> Promise;

    #[wasm_bindgen(js_name = uploadPinString4everland)]
    pub fn upload_pin_string_4everland(bucket_endpoint: String, pinning_url: String, bucket: String, folder: String, data: String, file_name: String, file_type: String, access_key: String, secret_access: String, session_token: String, pin_secret: String) -> Promise;

    #[wasm_bindgen(js_name = uploadPinBlob4everland)]
    pub fn upload_pin_file_4everland(bucket_endpoint: String, pinning_url: String, bucket: String, folder: String, blob: File, file_name: String, file_type: String, access_key: String, secret_access: String, session_token: String, pin_secret: String) -> Promise;

    #[wasm_bindgen(js_name = uploadPinBlob4everland)]
    pub fn upload_pin_blob_4everland(bucket_endpoint: String, pinning_url: String, bucket: String, folder: String, blob: Blob, file_name: String, file_type: String, access_key: String, secret_access: String, session_token: String, pin_secret: String) -> Promise;

}