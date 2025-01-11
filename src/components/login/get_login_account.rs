use leptos::prelude::*;

use crate::components::navigation::nav::Nav;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use leptos_use::storage::use_local_storage;

use codee::string::JsonSerdeCodec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccountState {
    pub account_id: String,
}

impl Default for AccountState {
    fn default() -> Self {
        Self {
            account_id: "".to_string(),
        }
    }
}

#[component]
pub fn GetLoginAccount() -> impl IntoView {
    let (account_state, set_account_state, reset_account) =
        use_local_storage::<AccountState, JsonSerdeCodec>("account-state");
    let (account_load, set_account_load) = signal(("".to_owned(), "".to_owned()));

    create_effect(move |_| {
        let (account_id, _) = account_load();
        if !account_id.is_empty() && account_id != account_state().account_id {
            set_account_state.update(|s| s.account_id = account_id.clone());
        }
    });
    view! {
        <div>
            <Nav/>
            <p>Loaded Account {account_load}</p>
            <div>
                <GetAccountsExtension set_account_load=set_account_load/>
            </div>
        </div>
    }
}
