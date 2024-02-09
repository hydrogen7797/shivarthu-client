use crate::components::accounts::accounts_handling::add_account::AddAccount;
use crate::components::home::Home;
use crate::components::markdown::markdown_component::MarkdownHtmlView;
use crate::components::schelling_game::profile_validation::add_profile::AddProfile;
use crate::components::schelling_game::profile_validation::extension_sign_in::ExtensionSignIn;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::components::upload::upload_video::FileUpload;
use leptos::*;
use leptos_router::*;

#[component]
pub fn RouterApp() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/add-account" view=AddAccount/>
                <Route path="/markdown" view=MarkdownHtmlView/>
                // <Route path="/upload" view=FileUpload/>
                <Route path="/add-profile" view=AddProfile/>
            // <Route path="/extension" view=ExtensionSignIn/>
            // <Route path="/signin" view=GetAccountsExtension/>
            </Routes>
        </Router>
    }
}
