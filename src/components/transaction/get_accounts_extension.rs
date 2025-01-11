use crate::services::common_services::{get_accounts, Account};
use crate::services::error::ErrorString;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;

async fn get_accounts_result() -> Result<Vec<Account>, ErrorString> {
    let accounts = get_accounts().await;
    accounts
}

#[component]
pub fn GetAccountsExtension(set_account_load: WriteSignal<(String, String)>) -> impl IntoView {
    let (accounts, set_accounts) = signal::<Vec<Account>>(vec![]);
    let (error, set_error) = signal(String::from(""));

    let onclick_button = move |_e: MouseEvent, i| {
        let accounts = accounts();
        let account: &Account = accounts.get(i).unwrap();
        let account_address = account.address.clone();
        let account_source = account.source.clone();
        set_account_load((account_address.clone(), account_source.clone()));
        gloo::console::log!(format!(
            "Account number{}, account address: {}, account source: {}",
            i, account_address, account_source
        ));
    };
    let get_accounts_click = move |_e: MouseEvent| {
        spawn_local({

    
            async move {
                match get_accounts_result().await {
                    Ok(result) => set_accounts(result),
                    Err(err) => set_error(err.0),
                }
            }
        });
    };
    let accounts_html = move || {
        let html_element = 
                    if accounts().is_empty() {
                        view! {
                            <div>
                                <br/>
                                <div>
                                    {"No Web3 extension accounts found. Install Talisman, SubWallet or the Polkadot.js extension and add an account."}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div>
                                <br/>
                                <div class="mb">
                                    <b>{"Select an account you want to use for signing:"}</b>
                                </div>
                                {move || {
                                    accounts()
                                        .iter()
                                        .enumerate()
                                        .map(|(i, account)| {
                                            view! {
                                                <div>
                                                    <button
                                                        class="btn btn-outline btn-info btn-block my-3"
                                                        on:click=move |e| { onclick_button(e, i) }
                                                        id=account.address.clone()
                                                    >
                                                        {account.source.clone()}
                                                        {" | "}
                                                        {account.name.clone()}
                                                        <br/>
                                                        <small>{account.address.clone()}</small>
                                                    </button>
                                                </div>
                                            }
                                        })
                                        .collect_view()
                                }}

                            </div>
                        }.into_any()
        };
        html_element
    };

    view! {
        <>
            <div class="container py-10 px-10 mx-0 min-w-full flex flex-col items-center">
                <div>
                    <button on:click=get_accounts_click class="btn btn-warning" id="select-account">
                        {"=> Select an Account for Signing"}
                    </button>
                </div>
                {move || accounts_html()}
            </div>
        </>
    }
}
