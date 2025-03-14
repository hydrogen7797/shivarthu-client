use leptos::prelude::*;

#[component]
pub fn Background() -> impl IntoView {
    view! {
        <section
            class="h-[600px] flex items-center justify-center"
            style="background-image: url('img/bg-masthead.jpg');
            background-size: cover;
            background-position: center;
            background-repeat: no-repeat;"
        >
            <div class="container mx-auto px-6">
                <div class="flex flex-col items-center text-center">
                    <h1 class="text-4xl font-bold mb-4 text-white bg-blue-700 hover:bg-blue-800 rounded-lg  px-5 py-2.5">
                        "Shivarthu"
                    </h1>
                </div>
            </div>
        </section>
    }
}
