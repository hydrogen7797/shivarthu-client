use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(department_required_fund_id: u64) -> impl IntoView {
    view! { <ExtensionSignIn department_required_fund_id={department_required_fund_id}/> }
}

#[component]
pub fn ExtensionSignIn(department_required_fund_id: u64) -> impl IntoView {
    let (account_load, set_account_load) = create_signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load={set_account_load}/>
                </div>
            }
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
        } else {
            view! { <div>{"Some Error Occured"}</div> }
        }
    };

    view! { <div>{move || render_html()}</div> }
}

#[component]
pub fn ExtensionTransaction(
    department_required_fund_id: u64,
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = create_signal(String::from(""));
    let (extrinsic_success, set_extrinsic_success) = create_signal(String::from(""));
    let transaction_resource = create_local_resource(
        move || {
            (
                department_required_fund_id.clone(),
                account_address.clone(),
                account_source.clone(),
                set_error,
                set_extrinsic_success,
            )
        },
        move |(
            department_required_fund_id,
            account_address,
            account_source,
            set_error,
            set_extrinsic_success,
        )| async move {
            let tx = polkadot::tx()
                .department_funding()
                .add_to_department_fund(department_required_fund_id);

            sign_in_with_extension(
                tx,
                account_address,
                account_source,
                set_error,
                set_extrinsic_success,
            )
            .await;
        },
    );

    let loading = transaction_resource.loading();
    let is_loading = move || {
        if loading() {
            view! {
                <div class="alert">
                    <span class="loading loading-spinner"></span>
                    "Loading... Please sign with extension."
                </div>
            }
        } else {
            view! { <div class="alert">"Idle."</div> }
        }
    };

    let error_fn = move || {
        if !error().is_empty() {
            view! {
                <div role="alert" class="alert alert-error">
                    {move || error()}
                </div>
            }
        } else {
            view! { <div></div> }
        }
    };

    let extrinsic_success_fn = move || {
        if !extrinsic_success().is_empty() {
            view! {
                <div role="alert" class="alert alert-success">
                    {move || extrinsic_success()}
                </div>
            }
        } else {
            view! { <div></div> }
        }
    };

    view! {
        <div class="md:container md:mx-auto">
            <div>{move || transaction_resource.get()}</div>
            <br/>
            <div>{move || is_loading()}</div>
            <br/>
            <div>{move || error_fn()}</div>
            <br/>
            <div>{move || extrinsic_success_fn()}</div>

        </div>
    }
}
