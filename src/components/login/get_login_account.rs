use leptos::*;

use crate::components::transaction::get_accounts_extension::GetAccountsExtension;

#[component]
pub fn GetLoginAccount() -> impl IntoView {
    let (account_load, set_account_load) = create_signal(("".to_owned(), "".to_owned()));
    view! {
        <div>
            <p>Loaded Account {account_load}</p>

            <div>
                <GetAccountsExtension set_account_load=set_account_load/>
            </div>
        </div>
    }
}
