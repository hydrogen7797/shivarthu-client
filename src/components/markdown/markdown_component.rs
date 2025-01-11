use crate::components::markdown::markdown_to_html::parse_text_to_html;
use leptos::ev::KeyboardEvent;
use leptos::prelude::*;

#[component]
pub fn MarkdownHtmlView() -> impl IntoView {
    let (html_data, set_html_data) = signal(String::from(""));
    let markdown_changed = move |e: KeyboardEvent| {
        let html = parse_text_to_html(&event_target_value(&e));
        gloo::console::log!(format!("{}", html));
        set_html_data(html);
    };

    view! {
        <>
            <div class="md:container md:mx-auto pt-20">
                <div>
                    <form>
                        <div class="mb-6">
                            <textarea
                                rows="10"
                                cols="50"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                on:keyup=markdown_changed
                            ></textarea>
                        </div>
                    </form>
                </div>
                <div inner_html=html_data></div>
            </div>
        </>
    }
}
