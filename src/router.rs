use crate::components::home::Home;
use crate::components::accounts::accounts_handling::add_account::AddAccount;
use leptos::*;
use leptos_router::*;

#[component]
pub fn RouterApp() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view={AddAccount}/>
                <Route path="/add-account" view={AddAccount}/>
            </Routes>
        </Router>
    }
}
