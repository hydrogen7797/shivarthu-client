use crate::services::common_services::{get_accounts, Account};
use crate::services::error::ErrorString;
use leptos::ev::MouseEvent;
use leptos::*;

async fn get_accounts_result() -> Result<Vec<Account>, ErrorString> {
    let accounts = get_accounts().await;
    accounts
}

#[component]
pub fn GetAccountsExtension(set_account_load: WriteSignal<(String, String)>) -> impl IntoView {
    let (accounts, set_accounts) = create_signal::<Vec<Account>>(vec![]);
    let get_accounts_action = create_action(|()| async move { get_accounts_result().await });
    let pending_accounts = get_accounts_action.pending();
    let get_accounts_value = get_accounts_action.value();

    let onclick_button = move |e: MouseEvent, i| {
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
    let get_accounts_click = move |e: MouseEvent| {
        get_accounts_action.dispatch(());
    };
    let accounts_html = move || {
        let accounts_option = get_accounts_value();
        let html_element = match accounts_option {
            Some(accounts_result) => match accounts_result {
                Ok(accounts_value) => {
                    if accounts_value.is_empty() {
                        view! {
                            <div>
                                <br/>
                                <div>
                                    {"No Web3 extension accounts found. Install Talisman, SubWallet or the Polkadot.js extension and add an account."}
                                </div>
                            </div>
                        }
                    } else {
                        set_accounts(accounts_value.clone());
                        view! {
                            <div>
                                <br/>
                                <div class="mb">
                                    <b>{"Select an account you want to use for signing:"}</b>
                                </div>
                                {move || {
                                    accounts_value
                                        .iter()
                                        .enumerate()
                                        .map(|(i, account)| {
                                            view! {
                                                <div>
                                                    <button
                                                        class="btn btn-outline btn-info btn-block my-3"
                                                        on:click=move |e| { onclick_button(e, i) }
                                                        id={&account.address}
                                                    >
                                                        {&account.source}
                                                        {" | "}
                                                        {&account.name}
                                                        <br/>
                                                        <small>{&account.address}</small>
                                                    </button>
                                                </div>
                                            }
                                        })
                                        .collect_view()
                                }}

                            </div>
                        }
                    }
                }
                Err(err) => view! {
                    <div>
                        <div class="error">{"Error: "} {format!("{:?}", err.0)}</div>
                    </div>
                },
            },
            None => view! { <div></div> },
        };
        html_element
    };

    view! {
        <>
            <div class="container py-10 px-10 mx-0 min-w-full flex flex-col items-center">
                <div>
                    <button on:click=get_accounts_click class="btn btn-warning">
                        {"=> Select an Account for Signing"}
                    </button>
                </div>
                {move || accounts_html()}
            </div>
        </>
    }
}
