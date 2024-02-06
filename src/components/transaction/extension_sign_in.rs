use crate::constants::constant::NODE_URL;
use crate::services::common_services::{extension_signature_for_extrinsic, polkadot, Account};
use anyhow::anyhow;
use leptos::*;
use std::ops::Deref;
use subxt::ext::codec::{Decode, Encode};
use subxt::tx::SubmittableExtrinsic;
use subxt::utils::{AccountId32, MultiSignature};
use subxt::{OnlineClient, PolkadotConfig};

pub fn sign_in_with_extension<T>(
    tx: T,
    account_address: String,
    account_source: String,
    set_error: WriteSignal<String>,
    set_extrinsic_error: WriteSignal<String>,
    set_extrinsic_success: WriteSignal<String>,
) where
    T: subxt::tx::TxPayload + 'static + std::clone::Clone + std::cmp::PartialEq,
{
    let async_transaction = create_resource(
        move || {
            (
                account_address.clone(),
                tx.clone(),
                account_source.clone(),
                set_error,
                set_extrinsic_error,
                set_extrinsic_success,
            )
        },
        |(
            account_address,
            tx,
            account_source,
            set_error,
            set_extrinsic_error,
            set_extrinsic_success,
        )| async move {
            let account_id: AccountId32 = account_address.parse().unwrap();

            let api = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                .await
                .unwrap();
            let account_nonce_option = match api.tx().account_nonce(&account_id).await {
                Ok(account_nonce) => Some(account_nonce),
                Err(_) => {
                    set_error("Fetching account nonce failed".to_string());
                    None
                }
            };
            let mut call_data_option = None;

            if account_nonce_option.is_some() {
                call_data_option = match api.tx().call_data(&tx) {
                    Ok(call_data) => Some(call_data),
                    Err(_) => {
                        set_error("could not encode call data".to_string());
                        None
                    }
                }
            }

            let mut signature_option = None;
            if account_nonce_option.is_some() && call_data_option.is_some() {
                signature_option = match extension_signature_for_extrinsic(
                    &call_data_option.unwrap(),
                    &api,
                    account_nonce_option.unwrap(),
                    account_source,
                    account_address,
                )
                .await
                {
                    Ok(signature) => Some(signature),
                    Err(_) => {
                        set_error("Signing via extension failed".to_string());
                        None
                    }
                }
            }

            let mut multi_signature_option = None;

            if signature_option.is_some() {
                let signature = signature_option.unwrap();
                multi_signature_option = match MultiSignature::decode(&mut &signature[..]) {
                    Ok(multi_signature) => Some(multi_signature),
                    Err(_) => {
                        set_error("MultiSignature Decoding".to_string());
                        None
                    }
                }
            }

            let mut partial_signed_option = None;
            if multi_signature_option.is_some() && account_nonce_option.is_some() {
                partial_signed_option = match api.tx().create_partial_signed_with_nonce(
                    &tx,
                    account_nonce_option.unwrap(),
                    Default::default(),
                ) {
                    Ok(partial_signed) => Some(partial_signed),
                    Err(_) => {
                        set_error("PartialExtrinsic creation failed".to_string());
                        None
                    }
                }
            }

            let mut signed_extrinsic_option = None;
            if partial_signed_option.is_some() && multi_signature_option.is_some() {
                let multi_signature = multi_signature_option.unwrap();
                let partial_signed = partial_signed_option.unwrap();
                let signed_extrinsic = partial_signed
                    .sign_with_address_and_signature(&account_id.into(), &multi_signature);
                signed_extrinsic_option = Some(signed_extrinsic);
            }

            if signed_extrinsic_option.is_some() {
                let signed_extrinsic = signed_extrinsic_option.unwrap();

                match submit_wait_finalized_and_get_extrinsic_success_event(signed_extrinsic).await
                {
                    Ok(remark_event) => {
                        set_extrinsic_success(format!("{:?}", remark_event));
                    }
                    Err(err) => {
                        set_extrinsic_error(err.to_string());
                    }
                }
            }
        },
    );
    create_effect(move |_| {
        async_transaction.get();
    });
}

async fn submit_wait_finalized_and_get_extrinsic_success_event(
    extrinsic: SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
) -> Result<polkadot::system::events::ExtrinsicSuccess, anyhow::Error> {
    let events = extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?;

    let events_str = format!("{:?}", &events);
    web_sys::console::log_1(&events_str.into());
    for event in events.find::<polkadot::system::events::ExtrinsicSuccess>() {
        web_sys::console::log_1(&format!("{:?}", event).into());
    }

    let success = events.find_first::<polkadot::system::events::ExtrinsicSuccess>()?;
    success.ok_or(anyhow!("ExtrinsicSuccess not found in events"))
}
