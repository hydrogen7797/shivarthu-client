use leptos::prelude::*;

#[component]
pub fn MilestoneTimeline() -> impl IntoView {
    view! {
        <div class="max-w-4xl mx-auto p-6 bg-white dark:bg-gray-900 shadow-lg rounded-2xl">
            <h2 class="text-3xl font-bold text-center text-gray-800 dark:text-gray-100 mb-6">
                "Project Milestone Timeline"
            </h2>
            <div class="relative border-l-4 border-gradient-to-b from-blue-500 via-purple-500 to-pink-500 pl-6 space-y-8">
                <Milestone title="Positive Externality" date="Early 2025" color="bg-blue-500" />
                <Milestone title="Profile Validation" date="Q3-Q4 2025" color="bg-purple-500" />
                <Milestone title="Department Funding" date="TBD" color="bg-green-500" />
                <Milestone
                    title="Election with Sequential Phragmén"
                    date="TBD"
                    color="bg-yellow-500"
                />
                <Milestone title="Anonymous Voting" date="TBD" color="bg-pink-500" />
            </div>
        </div>
    }
}

#[component]
pub fn Milestone(title: &'static str, date: &'static str, color: &'static str) -> impl IntoView {
    view! {
        <div class="relative">
            <div class={format!(
                "absolute -left-7 w-6 h-6 {} rounded-full border-4 border-white dark:border-gray-900",
                color,
            )}></div>
            <div class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg shadow">
                <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">{title}</h3>
                <p class="text-gray-600 dark:text-gray-300 text-sm">{date}</p>
            </div>
        </div>
    }
}
