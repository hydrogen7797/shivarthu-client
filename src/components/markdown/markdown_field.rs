use crate::components::markdown::markdown_to_html::parse_text_to_html;
use leptos::ev::{Event, KeyboardEvent};
use leptos::*;

#[component]
pub fn MarkdownField(
    class: String,
    name: String,
    #[prop(default = true)] required: bool,
    #[prop(into)] set_markdown: WriteSignal<String>,
) -> impl IntoView {
    let (html_data, set_html_data) = create_signal(String::from(""));
    let markdown_changed = move |e: KeyboardEvent| {
        let html = parse_text_to_html(&event_target_value(&e));
        // gloo::console::log!(format!("{}", html));
        set_html_data(html);
    };

    let handle_onchange = move |e: Event| {
        set_markdown(event_target_value(&e));
    };

    view! {
        <>
            <div>
                <div>
                    <form>
                        <div class="mb-6">
                            <textarea
                                rows="10"
                                cols="50"
                                name=name
                                required=required
                                class=format!("{}", class)

                                on:keyup=markdown_changed
                                on:input=handle_onchange
                            ></textarea>
                        </div>
                    </form>
                </div>
                <div inner_html=html_data></div>
            </div>
        </>
    }
}
