use crate::components::accounts::accounts_handling::add_account::AddAccount;
use crate::components::accounts::set_phase_from_pass::SetPhraseFromPass;
use crate::components::balance::transfer_balance::TransferBalance;
use crate::components::home::Home;
use crate::components::markdown::markdown_component::MarkdownHtmlView;
use crate::components::schelling_game::department_funding::{
    apply_staking_period_params::ApplyStakingPeriodParams as ApplyStakingPeriodForDepartmentFunding,
    change_period_sign_in::SignTransaction as ChangePeriodDepartmentFunding,
    create_department_fund::CreateDepartmentFund,
    game::schelling_game::SchellingGame as DepartmentFundingSchellingGame,
    views::juror_selected_check::JurorSelectedCheck as JurorSelectedCheckDepartmentFunding,
};
use crate::components::schelling_game::positive_externality::{
    apply_staking_period_params::ApplyStakingPeriodParams as ApplyStakingPeriodForPositiveExternality,
    change_period_sign_in::SignTransaction as ChangePeriodPositiveExternality,
    create_post::CreatePositiveExternalityPost,
    game::schelling_game::SchellingGame as PositiveExternalitySchellingGame,
    home::PositiveExternalityHome,
    views::juror_selected_check::JurorSelectedCheck as JurorSelectedCheckPositiveExternality,
};
use crate::components::schelling_game::profile_validation::add_profile::AddProfile;

use crate::components::schelling_game::profile_validation::add_profile_stake::AddProfileStake;

use crate::components::schelling_game::profile_validation::change_period_sign_in::SignTransaction as ChangePeriodProfileValidation;
use crate::components::schelling_game::profile_validation::game::schelling_game::SchellingGame;

use crate::components::login::get_login_account::GetLoginAccount;
use crate::components::schelling_game::departments::create_department::CreateDepartment;
use crate::components::schelling_game::profile_validation::view_profile_from_address::ViewProfileFromAddress;
use crate::components::schelling_game::profile_validation::views::juror_selected_check::JurorSelectedCheck;
use crate::components::schelling_game::project_tips::{
    apply_staking_period_params::ApplyStakingPeriodParams as ApplyStakingPeriodForProjectTips,
    change_period_sign_in::SignTransaction as ChangePeriodProjectTips,
    create_project::CreateProject, game::schelling_game::SchellingGame as ProjectTipsSchellingGame,
    views::juror_selected_check::JurorSelectedCheck as JurorSelectedCheckProjectTips,
};
use crate::components::tests::block_number::BlockNumber;
use crate::components::tests::display_error::NumericInput;
use crate::components::tests::polkadotjs_test::Polkadotjs;

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

                // Project tips routes

                <Route path="/project-tips/create-project/:department_id" view=CreateProject/>
                <Route
                    path="/project-tips/apply-staking-period/:project_id"
                    view=ApplyStakingPeriodForProjectTips
                />
                <Route
                    path="/project-tips/schelling-game/:project_id"
                    view=ProjectTipsSchellingGame
                />
                <Route path="/project-tips-change-period/:project_id" view=ChangePeriodProjectTips/>
                <Route
                    path="/project-tips-juror-selected/:project_id"
                    view=JurorSelectedCheckProjectTips
                />

                // Positive externality routes

                <Route path="/positive-externality" view=PositiveExternalityHome/>
                <Route path="/positive-externality/create-post" view=CreatePositiveExternalityPost/>
                <Route
                    path="/positive-externality/apply-staking-period/:user_to_calculate"
                    view=ApplyStakingPeriodForPositiveExternality
                />

                <Route
                    path="/positive-externality/schelling-game/:user_to_calculate"
                    view=PositiveExternalitySchellingGame
                />

                <Route
                    path="/positive-externality-change-period/:user_to_calculate"
                    view=ChangePeriodPositiveExternality
                />
                <Route
                    path="/positive-externality-juror-selected/:user_to_calculate"
                    view=JurorSelectedCheckPositiveExternality
                />

                // Department funding routes
                <Route
                    path="/department-funding/create-department-fund/:department_id"
                    view=CreateDepartmentFund
                />

                <Route
                    path="/department-funding/apply-staking-period/:department_id"
                    view=ApplyStakingPeriodForDepartmentFunding
                />
                <Route
                    path="/department-funding/schelling-game/:department_id"
                    view=ProjectTipsSchellingGame
                />
                <Route
                    path="/department-funding-change-period/:department_id"
                    view=ChangePeriodDepartmentFunding
                />
                <Route
                    path="/department-funding-juror-selected/:department_id"
                    view=JurorSelectedCheckDepartmentFunding
                />

                // Create department
                <Route path="/create-department" view=CreateDepartment/>

                // Login account
                <Route path="/get-login-account" view=GetLoginAccount/>
            </Routes>
        </Router>
    }
}
