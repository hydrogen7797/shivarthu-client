use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (nav_open, set_nav_open) = signal(false);

    view! {
        {}
        <nav class="bg-white border-gray-200 dark:bg-gray-900 dark:border-gray-700">
            <div class="max-w-screen-xl mx-auto flex items-center justify-between p-4">

                {} <a href="#" class="text-xl font-semibold dark:text-white">
                    "Shivarthu"
                </a>
                {}
                <button
                    on:click=move |_| set_nav_open.update(|n| *n = !*n)
                    class="lg:hidden inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
                >
                    <span class="sr-only">"Toggle Menu"</span>
                    <svg
                        class="w-5 h-5"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 17 14"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M1 1h15M1 7h15M1 13h15"
                        ></path>
                    </svg>
                </button>
                {} <div class="hidden lg:flex space-x-8">{navbar_items()}</div>
            </div>

            {}
            <div class=move || { if nav_open() { "block lg:hidden" } else { "hidden lg:hidden" } }>

                <div class="px-4 py-3 space-y-2 text-xl">{navbar_items()}</div>
            </div>
        </nav>
    }
}

fn navbar_items() -> impl IntoView {
    let (submenu_open, set_submenu_open) = signal(false);

    let (disobedience_open, set_disobedience_open) = signal(false);

    view! {
        <>
            <a
                href="/"
                class="block py-2 px-4 text-gray-700 rounded hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
            >
                "Home"
            </a>
            <div class="relative">
                <button
                    on:click=move |_| set_submenu_open.update(|n| *n = !*n)
                    class="block w-full text-left py-2 px-4 text-gray-700 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                >
                    "Profile Validation"
                    <svg
                        class="inline w-4 h-4 ml-2"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 10 6"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M1 1l4 4 4-4"
                        ></path>
                    </svg>
                </button>
                <div class=move || {
                    if submenu_open() {
                        "relative w-full mt-2 space-y-1 bg-white rounded shadow dark:bg-gray-800 lg:absolute lg:w-auto"
                    } else {
                        "hidden"
                    }
                }>

                    <a
                        href="/"
                        class="block py-2 w-full px-4 text-gray-700 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                    >
                        "Profile 1"
                    </a>
                    <a
                        href="/"
                        class="block py-2 w-full px-4 text-gray-700 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                    >
                        "Profile 2"
                    </a>
                    <a
                        href="/"
                        class="block py-2 w-full px-4 text-gray-700 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                    >
                        "Profile 3"
                    </a>
                </div>
            </div>

            <div class="relative">
                <button
                    on:click=move |_| set_disobedience_open.update(|n| *n = !*n)
                    class="block w-full text-left py-2 px-4 text-gray-700 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                >
                    "Departments"
                    <svg
                        class="inline w-4 h-4 ml-2"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 10 6"
                    >
                        <path
                            stroke="currentColor"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M1 1l4 4 4-4"
                        ></path>
                    </svg>
                </button>
                <div class=move || {
                    if disobedience_open() {
                        "relative w-full mt-2 space-y-1 bg-white rounded shadow dark:bg-gray-800 lg:absolute lg:w-auto"
                    } else {
                        "hidden"
                    }
                }>

                    <a
                        href="/"
                        class="block py-2 w-full px-4 text-gray-700 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                    >
                        "Create department"
                    </a>
                </div>
            </div>
            <a
                href="/"
                class="block py-2 px-4 text-gray-700 rounded hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
            >
                "Positive Externality"
            </a>
        </>
    }
}