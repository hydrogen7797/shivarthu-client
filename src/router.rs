use crate::components::accounts::accounts_handling::add_account::AddAccount;
use crate::components::accounts::set_phase_from_pass::SetPhraseFromPass;
use crate::components::balance::transfer_balance::TransferBalance;
use crate::components::home::Home;
use crate::components::markdown::markdown_component::MarkdownHtmlView;
use crate::components::schelling_game::positive_externality::{
    apply_staking_period_params::ApplyStakingPeriodParams,
    create_post::CreatePositiveExternalityPost,
    game::schelling_game::SchellingGame as PositiveExternalitySchellingGame,
    home::PositiveExternalityHome,
};
use crate::components::schelling_game::profile_validation::add_profile::AddProfile;
use crate::components::schelling_game::profile_validation::add_profile_sign_in::ExtensionSignIn;
use crate::components::schelling_game::profile_validation::add_profile_stake::AddProfileStake;
use crate::components::schelling_game::profile_validation::apply_jurors::ApplyJurors;
use crate::components::schelling_game::profile_validation::change_period_sign_in::SignTransaction as ChangePeriodProfileValidation;
use crate::components::schelling_game::profile_validation::game::schelling_game::SchellingGame;
use crate::components::schelling_game::profile_validation::rpc::evidence_end_block::EvidenceEndBlock;
use crate::components::schelling_game::profile_validation::view_profile_from_address::ViewProfileFromAddress;
use crate::components::schelling_game::profile_validation::views::juror_selected_check::JurorSelectedCheck;
use crate::components::schelling_game::project_tips::create_project::CreateProject;
use crate::components::tests::block_number::BlockNumber;
use crate::components::tests::display_error::NumericInput;
use crate::components::tests::polkadotjs_test::Polkadotjs;
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
                <Route path="/view-profile/:profile_user_account" view=ViewProfileFromAddress/>
                <Route path="/add-profile-stake/:profile_user_account" view=AddProfileStake/>
                <Route path="/profile-validation-game/:profile_user_account" view=SchellingGame/>
                // <Route path="/extension" view=ExtensionSignIn/>
                // <Route path="/signin" view=GetAccountsExtension/>
                // <Route path="/apply-juror/:profile_user_account" view=ApplyJurors/>
                <Route path="/transfer-balance" view=TransferBalance/>
                <Route path="/block-number" view=BlockNumber/>
                <Route path="/juror-selected/:profile_user_account" view=JurorSelectedCheck/>
                <Route path="/profile-validation/:profile_user_account" view=SchellingGame/>
                <Route path="/error-handling" view=NumericInput/>
                <Route path="/enter-password" view=SetPhraseFromPass/>
                <Route path="/polkadotjs" view=Polkadotjs/>
                <Route
                    path="/profile-validation-change-period/:profile_user_account"
                    view=ChangePeriodProfileValidation
                />
                <Route path="/project-tips/create-project/:department_id" view=CreateProject/>
                <Route path="/positive-externality" view=PositiveExternalityHome/>
                <Route path="/positive-externality/create-post" view=CreatePositiveExternalityPost/>
                <Route
                    path="/positive-externality/apply-staking-period/:user_to_calculate"
                    view=ApplyStakingPeriodParams
                />

                <Route
                    path="/positive-externality/schelling-game/:user_to_calculate"
                    view=PositiveExternalitySchellingGame
                />

            </Routes>
        </Router>
    }
}
