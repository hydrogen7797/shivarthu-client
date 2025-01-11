use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::prelude::*;

#[component]
pub fn Polkadotjs() -> impl IntoView {
    view! { <ExtensionSignIn/> }
}

#[component]
pub fn ExtensionSignIn() -> impl IntoView {
    let (account_load, set_account_load) = signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load=set_account_load/>
                </div>
            }.into_any()
        } else if !account_load().0.is_empty() && !account_load().1.is_empty() {
            view! {
                <div>
                    <ExtensionTransaction
                        account_address=account_load().0
                        account_source=account_load().1
                    />
                </div>
            }.into_any()
        } else {
            view! { <div>{"Some Error Occured"}</div> }.into_any()
        }
    };

    view! { <div>{move || render_html()}</div> }
}

async fn load_data(account_address: String, account_source: String, set_error:WriteSignal<String>, set_extrinsic_success:WriteSignal<String>) {
    let tx = polkadot::tx().department_funding().pass_period(10);

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
pub fn ExtensionTransaction(account_address: String, account_source: String) -> impl IntoView {
    let (error, set_error) = signal(String::from("hello"));
    let (extrinsic_success, set_extrinsic_success) = signal(String::from("extrinsic"));
    let transaction_resource = LocalResource::new(move ||{
        load_data(account_address.clone(), account_source.clone(), set_error, set_extrinsic_success)
    });

    let async_result = move || {
        transaction_resource
            .get()
            .as_deref()
            .map(|_| view! { <div></div> }.into_any())
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
