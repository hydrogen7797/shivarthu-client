use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccountStore {
    pub hash: Option<String>,
    pub account_address: Option<String>,
}

impl Default for AccountStore {
    fn default() -> Self {
        Self {
            hash: None,
            account_address: None,
        }
    }
}




// Does not get stored in local storage
#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub (super) struct PhraseStore {
    pub mnemonic_phrase: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct PhaseExists {
    pub phase_exists_in_state: bool,
}