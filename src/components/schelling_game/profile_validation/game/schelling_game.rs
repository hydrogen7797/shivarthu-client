use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::profile_validation::apply_jurors::ApplyJurors;

use crate::components::schelling_game::profile_validation::challenge_evidence::ChallengeEvidence;

use crate::components::schelling_game::profile_validation::commit_vote::CommitVote;
use crate::components::schelling_game::profile_validation::draw_jurors::DrawJurors;
use crate::components::schelling_game::profile_validation::reveal_vote::RevealVote;
use crate::components::schelling_game::profile_validation::storage::get_period_fn::get_period_fn;
use crate::services::common_services::polkadot::runtime_types::pallet_schelling_game_shared::types::Period;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn SchellingGame() -> impl IntoView {
    let params = use_params_map();

    let profile_user_account =
        move || params.with(|params| params.get("profile_user_account").unwrap_or_default());

    let account = untrack(move || profile_user_account());

    view! {
        <div>
            <SchellingGameComponent profile_user_account={account} />
        </div>
    }
}

#[component]
pub fn SchellingGameComponent(profile_user_account: String) -> impl IntoView {
    let profile_user_account = move || profile_user_account.clone();

    let period = get_period_fn(profile_user_account());

    let myview = move || {
        {
            {
                // let period_read_signal = period();
                if let Some(period) = period() {
                    let view = match period {
                        Period::Evidence => view! {
                            <div>

                                <ChallengeEvidence profile_user_account={profile_user_account()} />

                            </div>
                        }
                        .into_any(),
                        Period::Staking => view! {
                            <div>
                                <ApplyJurors profile_user_account={profile_user_account()} />
                            </div>
                        }
                        .into_any(),
                        Period::Drawing => view! {
                            <div>
                                <DrawJurors profile_user_account={profile_user_account()} />
                            </div>
                        }
                        .into_any(),
                        Period::Commit => view! {
                            <div>
                                <CommitVote profile_user_account={profile_user_account()} />
                            </div>
                        }
                        .into_any(),
                        Period::Vote => view! {
                            <div>
                                <RevealVote profile_user_account={profile_user_account()} />
                            </div>
                        }
                        .into_any(),
                        Period::Appeal => view! { <div></div> }.into_any(),
                        Period::Execution => {
                            view! { <div>You are in Execution phase. Get your incentives</div> }
                        }
                        .into_any(),
                    };
                    view
                } else {
                    view! {
                        <div class="container">
                            <p>{format!("{:?}", period())}</p>
                            <p>{"No period"}</p>
                        </div>
                    }
                    .into_any()
                }
            }
        }
    };

    view! {
        <div>
            <Nav />
            // {move || account()}
            // {move || format!("{:?}", period())}
            {move || myview()}
        </div>
    }
}
