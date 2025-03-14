use leptos::prelude::*;

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <div class="p-8 text-gray-100 dark:bg-gradient-to-r dark:from-gray-900 dark:to-black">
            <div class="max-w-6xl mx-auto bg-white dark:bg-gray-800 p-6 rounded-xl shadow-lg">
                <h1 class="text-4xl font-extrabold text-center text-yellow-400 dark:text-yellow-300 mb-6 sm:block hidden">
                    "Participate in a New Experiment of Direct Democracy: Fair and Inclusive"
                </h1>
                <h1 class="text-3xl font-extrabold text-center text-yellow-400 dark:text-yellow-300 mb-6 sm:hidden">
                    "Direct Democracy: Fair and Inclusive"
                </h1>
                <h2 class="text-xl font-extrabold text-center text-yellow-400 dark:text-yellow-300 mb-6">
                    "Shivarthu's Vision or Goals for Achieving: Building a Transparent, Collaborative, and Future-Ready Democracy"
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
                    <DemocracyCard
                        title={"Decentralized Democracy".to_string()}
                        description={"No authoritarian regime or anarchical regime, and no concentration of power. The ability to approve policy is limited only to the concerned department with the required expertise."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"Evidence-based Democracy".to_string()}
                        description={"Only evidence-based policies are allowed, and representatives must be experts in the field with statements backed by evidence and sound reasoning."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"No Catch-22 Paradox".to_string()}
                        description={"True freedom to select nominees for election, independent of political parties, ensuring a broad selection and easy removal of representatives without vested interests."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"Collaborative Democracy".to_string()}
                        description={"Problem-solving through collaboration, not competition. Representatives must work together as they can be removed if they act frivolously."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"Infinitely Stable Democracy".to_string()}
                        description={"A government that remains stable indefinitely with voting intervals of every six months."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"No Nepotism".to_string()}
                        description={"Representatives favoring relatives or friends cannot sustain as frivolous ones are removed."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"No Conflict of Interest Democracy".to_string()}
                        description={"Representatives act independently without alliance biases. The system is designed to prevent conflicts of interest."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"Evolutionary Democracy".to_string()}
                        description={"Democracy that refines itself every six months, improving efficiency as bad candidates are removed."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"Blockchain Democracy".to_string()}
                        description={"Democracy without middlemen like politicians and bureaucrats, allowing direct interaction with service providers."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"Mega Participation Democracy".to_string()}
                        description={"A large number of expert representatives implement policies collaboratively and assign tasks to citizens and students for rapid execution."
                            .to_string()}
                    />
                    <DemocracyCard
                        title={"Mobile and Global Democracy".to_string()}
                        description={"A democracy where national boundaries fade over time, focusing on expertise rather than origin."
                            .to_string()}
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn DemocracyCard(title: String, description: String) -> impl IntoView {
    view! {
        <div class="bg-gray-100 dark:bg-gray-700 p-4 rounded-lg shadow-md text-center">
            <h2 class="text-2xl font-bold text-blue-600 dark:text-blue-300">{title.clone()}</h2>
            <p class="text-gray-800 dark:text-gray-300 mt-2">{description.clone()}</p>
        </div>
    }
}
