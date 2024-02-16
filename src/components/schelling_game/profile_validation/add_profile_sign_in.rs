use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::*;
use polkadot::runtime_types::pallet_support::Content;

#[component]
pub fn SignTransaction(post_cid: ReadSignal<String>) -> impl IntoView {
    view! { <ExtensionSignIn post_cid=post_cid/> }
}

#[component]
pub fn ExtensionSignIn(post_cid: ReadSignal<String>) -> impl IntoView {
    let (account_load, set_account_load) = create_signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        {
            move || {
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
                                post_cid=post_cid
                                account_address=account_load().0
                                account_source=account_load().1
                            />
                        </div>
                    }
                } else {
                    view! { <div>{"Some Error Occured"}</div> }
                }
            }
        }
    };
    view! { <div>{move || render_html()}</div> }
}

#[component]
pub fn ExtensionTransaction(
    post_cid: ReadSignal<String>,
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = create_signal(String::from("hello"));
    let (extrinsic_success, set_extrinsic_success) = create_signal(String::from("extrinsic"));
    let (account, set_account) = create_signal((account_address, account_source));
    let transaction_resource = create_local_resource(
        move || (post_cid, account, set_error, set_extrinsic_success),
        move |_| async move {
            let content: Content = Content::IPFS(post_cid().as_bytes().to_vec());
            let tx = polkadot::tx().profile_validation().add_citizen(content);
            let (account_address, account_source) = account();
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
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! {
        <div>
            <div>{move || transaction_resource.get()}</div>
            <div>{move || is_loading()}</div>
            <div>{move || error()}</div>
            <div>{move || extrinsic_success()}</div>
        </div>
    }
}
