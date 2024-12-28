use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(dest_account: String, transfer_balance: u128) -> impl IntoView {
    view! { <ExtensionSignIn dest_account={dest_account} transfer_balance={transfer_balance}/> }
}

#[component]
pub fn ExtensionSignIn(dest_account: String, transfer_balance: u128) -> impl IntoView {
    let (account_load, set_account_load) = signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load={set_account_load}/>
                </div>
            }.into_any()
        } else if !account_load().0.is_empty() && !account_load().1.is_empty() {
            view! {
                <div>
                    <ExtensionTransaction
                        dest_account={dest_account.clone()}
                        transfer_balance={transfer_balance}
                        account_address={account_load().0}
                        account_source={account_load().1}
                    />
                </div>
            }.into_any()
        } else {
            view! { <div>{"Some Error Occured"}</div> }.into_any()
        }
    };

    view! { <div>{move || render_html()}</div> }
}

async fn transaction(
    dest_account: String,
    transfer_balance: u128,
    account_address: String,
    account_source: String,
    set_error:WriteSignal<String>,
    set_extrinsic_success:WriteSignal<String>
) {

    let account_id32 = AccountId32::from_str(&dest_account).unwrap();

            let tx = polkadot::tx().balances().transfer_allow_death(
                subxt::utils::MultiAddress::Id(account_id32),
                transfer_balance,
            );
            sign_in_with_extension(
                tx,
                account_address,
                account_source,
                set_error,
                set_extrinsic_success,
            )
            .await;

}

#[component]
pub fn ExtensionTransaction(
    dest_account: String,
    transfer_balance: u128,
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = signal(String::from("hello"));
    let (extrinsic_success, set_extrinsic_success) = signal(String::from("extrinsic"));
    let transaction_resource = LocalResource::new(
        move || transaction
            (
                dest_account.clone(),
                transfer_balance,
                account_address.clone(),
                account_source.clone(),
                set_error,
                set_extrinsic_success,
            )

    );

    let async_result = move || {
        transaction_resource
            .get()
            .as_deref()
            .map(|_| view!{<div></div>}.into_any())
            // This loading state will only show before the first load
            .unwrap_or_else(|| view! {
                <div class="alert">
                    <span class="loading loading-spinner"></span>
                    "Loading... Please sign with extension."
                </div>
            }
            .into_any())
    };

    view! {
        <div>
            <div>{async_result()}</div>
            <div>{move || error()}</div>
            <div>{move || extrinsic_success()}</div>
        </div>
    }
}
