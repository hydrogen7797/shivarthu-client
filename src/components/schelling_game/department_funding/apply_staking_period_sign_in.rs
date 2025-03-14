use crate::components::common::spinner::LoadingSpinner;
use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(department_required_fund_id: u64) -> impl IntoView {
    view! { <ExtensionSignIn department_required_fund_id={department_required_fund_id} /> }
}

#[component]
pub fn ExtensionSignIn(department_required_fund_id: u64) -> impl IntoView {
    let (account_load, set_account_load) = signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load={set_account_load} />
                </div>
            }
            .into_any()
        } else if !account_load().0.is_empty() && !account_load().1.is_empty() {
            view! {
                <div>
                    <ExtensionTransaction
                        department_required_fund_id={department_required_fund_id.clone()}
                        account_address={account_load().0}
                        account_source={account_load().1}
                    />
                </div>
            }
            .into_any()
        } else {
            view! { <div>{"Some Error Occured"}</div> }.into_any()
        }
    };

    view! { <div>{move || render_html()}</div> }
}

async fn transaction(
    department_required_fund_id: u64,
    account_address: String,
    account_source: String,
    set_error: WriteSignal<String>,
    set_extrinsic_success: WriteSignal<String>,
) {
    let tx = polkadot::tx()
        .department_funding()
        .apply_staking_period(department_required_fund_id);

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
    department_required_fund_id: u64,
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = signal(String::from(""));
    let (extrinsic_success, set_extrinsic_success) = signal(String::from(""));
    let transaction_resource = LocalResource::new(move || {
        transaction(
            department_required_fund_id.clone(),
            account_address.clone(),
            account_source.clone(),
            set_error,
            set_extrinsic_success,
        )
    });

    let async_result = move || {
        transaction_resource
            .get()
            .as_deref()
            .map(|_| view! { <div></div> }.into_any())
            // This loading state will only show before the first load
            .unwrap_or_else(|| view! {
                <div class="flex items-center gap-3 p-4 border-l-4 border-yellow-500 bg-yellow-100 text-yellow-800 rounded-xl shadow-md">
                    <div>
                        <LoadingSpinner />
                    </div>
                    <div>"Loading... Please sign with extension."</div>
                </div>
            }
            .into_any())
    };
    let error_fn = move || {
        if !error().is_empty() {
            view! {
                <div
                    role="alert"
                    class="flex items-center gap-3 p-4 border-l-4 border-red-500 bg-red-100 text-red-800 rounded-xl shadow-md"
                >
                    {move || error()}
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    };

    let extrinsic_success_fn = move || {
        if !extrinsic_success().is_empty() {
            view! {
                <div
                    role="alert"
                    class="flex items-center gap-3 p-4 border-l-4 border-green-500 bg-green-100 text-green-800 rounded-xl shadow-md"
                >
                    {move || extrinsic_success()}
                </div>
            }.into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    };

    view! {
        <div class="md:container md:mx-auto">
            <div>{async_result}</div>
            <br />
            <br />
            <div>{move || error_fn()}</div>
            <br />
            <div>{move || extrinsic_success_fn()}</div>

        </div>
    }
}
