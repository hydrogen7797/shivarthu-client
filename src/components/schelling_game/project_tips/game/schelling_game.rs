use crate::components::navigation::nav::Nav;
use crate::components::schelling_game::project_tips::apply_jurors::ApplyJurors;

use crate::components::schelling_game::project_tips::commit_vote::CommitVote;
use crate::components::schelling_game::project_tips::draw_jurors::DrawJurors;
use crate::components::schelling_game::project_tips::reveal_vote::RevealVote;
use crate::components::schelling_game::project_tips::storage::get_period_fn::get_period_fn;
use crate::services::common_services::polkadot::runtime_types::pallet_schelling_game_shared::types::Period;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn SchellingGame() -> impl IntoView {
    let params = use_params_map();

    let project_id = move || {
        params.with(|params| {
            params
                .get("project_id")
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or_default()
        })
    };

    let account = untrack(move || project_id());

    view! {
        <div>
            <SchellingGameComponent project_id={account} />
        </div>
    }
}

#[component]
pub fn SchellingGameComponent(project_id: u64) -> impl IntoView {
    let project_id = move || project_id.clone();

    let period = get_period_fn(project_id());

    let myview = move || {
        {
            {
                // let period_read_signal = period();
                if let Some(period) = period() {
                    let view = match period {
                        Period::Evidence => view! { <div></div> }.into_any(),
                        Period::Staking => view! {
                            <div>
                                <ApplyJurors project_id={project_id()} />
                            </div>
                        }
                        .into_any(),
                        Period::Drawing => view! {
                            <div>
                                <DrawJurors project_id={project_id()} />
                            </div>
                        }
                        .into_any(),
                        Period::Commit => view! {
                            <div>
                                <CommitVote project_id={project_id()} />
                            </div>
                        }
                        .into_any(),
                        Period::Vote => view! {
                            <div>
                                <RevealVote project_id={project_id()} />
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
