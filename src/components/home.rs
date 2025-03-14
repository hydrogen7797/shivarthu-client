use crate::components::navigation::nav::Nav;
use crate::pages::background::Background;
use crate::pages::features::Features;
use crate::pages::milestone::MilestoneTimeline;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <>
            <Nav />

            <Background />

            // <!-- Hero Section -->
            <section class="bg-white dark:bg-slate-800 py-12">
                <div class="container mx-auto px-6">
                    <div class="flex flex-col items-center">
                        <h1 class="text-4xl font-bold text-gray-800 mb-4 text-2xl dark:text-white">
                            "Shivarthu: A Decentralized Governance"
                        </h1>
                        <p class="text-gray-600 mb-8 text-lg dark:text-white">
                            "Welcome to Shivarthu, a platform that empowers individuals to take part in community governance while earning funding for their contributions. Our decentralized governance model allows for transparent and equitable decision-making, ensuring that everyone has a voice in shaping the future of their community. By participating in our governance process, you can help drive forward initiatives that matter to you and make a real impact. Plus, with our funding incentives, you'll be rewarded for your dedication and hard work. Join us today and be a part of something truly transformative. Together, we can build stronger, more resilient communities that thrive on the power of collective action."
                        </p>
                        <a
                            href="#"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >
                            Test Net
                        </a>
                    </div>
                </div>
            </section>

            <section class="py-12">
                <div class="container mx-auto px-6">
                    <div class="grid grid-cols-1  sm:grid-cols-1 md:grid-cols-1 lg:grid-cols-3 gap-6">
                        <div class="bg-white p-6 rounded-lg shadow-md dark:bg-slate-800">
                            <div class="flex items-center mb-4">
                                <div class="flex items-center justify-center w-12 h-12 bg-green-500 rounded-full mr-3">
                                    <span class="flex items-center justify-center w-6 h-6 font-bold text-white ounded-full text-xs">
                                        1
                                    </span>
                                </div>

                                <h3 class="text-xl font-bold dark:text-white">
                                    "Authentication of Users"
                                </h3>
                            </div>
                            <p class="text-gray-600 text-lg dark:text-white">
                                "User authentication is a critical component of our platform, ensuring that only genuine individuals can participate in community governance. We employ robust verification processes to confirm the existence and authenticity of each user, safeguarding against Sybil attacks and ensuring that every voice is legitimate."
                            </p>
                        </div>
                        <div class="bg-white dark:bg-slate-800 p-6 rounded-lg shadow-md">
                            <div class="flex items-center mb-4">
                                <div class="flex items-center justify-center w-12 h-12 bg-green-500 rounded-full mr-3">
                                    <span class="flex items-center justify-center w-6 h-6 font-bold text-white  rounded-full text-xs">
                                        2
                                    </span>
                                </div>
                                <h3 class="text-xl font-bold dark:text-white">
                                    "Empower Your Team"
                                </h3>
                            </div>
                            <p class="text-gray-600 text-lg dark:text-white">
                                "Empower your team to make a real impact with Shivarthu's department creation feature. By establishing your own department within our platform, you unlock access to dedicated funding streams specifically allocated for your team's initiatives. This enables you to pursue projects and endeavors that align with your unique passions and expertise, all while contributing to the broader community."
                            </p>
                        </div>
                        <div class="bg-white dark:bg-slate-800 p-6 rounded-lg shadow-md">
                            <div class="flex items-center mb-4">
                                <div class="flex items-center justify-center w-12 h-12 bg-green-500 rounded-full mr-3">
                                    <span class="flex items-center justify-center w-6 h-6 font-bold text-white  rounded-full text-xs">
                                        3
                                    </span>
                                </div>
                                <h3 class="text-xl font-bold dark:text-white">"Wisdom of Crowd"</h3>
                            </div>
                            <p class="text-gray-600 text-lg dark:text-white">
                                "A novel approach to price discovery using wisdom of the crowd where sortition ensures a diverse and random selection of participants, representing a broad spectrum of perspectives. Through the process of Schelling games, these individuals collectively converge on a price, reflecting a consensus formed by the group. This decentralized mechanism reduces the influence of any single participant."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            <MilestoneTimeline />

            <Features />

            // <!-- Footer -->
            <footer class="bg-white dark:bg-slate-800 py-6">
                <div class="container mx-auto px-6 text-center">
                    <p class="text-gray-600 dark:text-white">
                        "2024, Shivarthu,  This work is licensed under a "
                        <a rel="license" href="https://creativecommons.org/licenses/by/4.0/">
                            "Creative Commons Attribution 4.0 License"
                        </a>
                    </p>
                </div>
            </footer>
        </>
    }
}
