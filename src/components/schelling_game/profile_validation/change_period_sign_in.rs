use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::*;
use leptos_router::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction() -> impl IntoView {
    let params = use_params_map();

    let profile_user_account = move || {
        params.with(|params| {
            params
                .get("profile_user_account")
                .cloned()
                .unwrap_or_default()
        })
    };

    view! { <ExtensionSignIn profile_user_account=profile_user_account()/> }
}

#[component]
pub fn ExtensionSignIn(profile_user_account: String) -> impl IntoView {
    let (account_load, set_account_load) = create_signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load=set_account_load/>
                </div>
            }
        } else if !account_load().0.is_empty() && !account_load().1.is_empty() {
            view! {
                <div>
                    <ExtensionTransaction
                        profile_user_account=profile_user_account.clone()
                        account_address=account_load().0
                        account_source=account_load().1
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
    profile_user_account: String,
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = create_signal(String::from("hello"));
    let (extrinsic_success, set_extrinsic_success) = create_signal(String::from("extrinsic"));
    let transaction_resource = create_local_resource(
        move || {
            (
                profile_user_account.clone(),
                account_address.clone(),
                account_source.clone(),
                set_error,
                set_extrinsic_success,
            )
        },
        move |(
            profile_user_account,
            account_address,
            account_source,
            set_error,
            set_extrinsic_success,
        )| async move {
            let account_id32 = AccountId32::from_str(&profile_user_account.clone()).unwrap();

            let tx = polkadot::tx()
                .profile_validation()
                .pass_period(account_id32);

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
            "Loading... Please sign with extension."
        } else {
            "Idle."
        }
    };

    view! {
        <div>
            <div>{move || transaction_resource.get()}</div>
            <div>{move || is_loading()}</div>
            <div>{move || error()}</div>
            <div>{move || extrinsic_success()}</div>
        </div>
    }
}
