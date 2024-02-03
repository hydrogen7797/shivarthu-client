use crate::components::markdown::markdown_field::MarkdownField;
use crate::components::navigation::nav::Nav;
use crate::components::upload::upload_video::FileUpload;
use json::object;
use leptos::ev::SubmitEvent;
use leptos::*;

#[component]
pub fn AddProfile() -> impl IntoView {
    let (name, set_name) = create_signal(String::from(""));
    let (markdown, set_markdown) = create_signal(String::from(""));
    let (cid, set_cid) = create_signal(String::from(""));

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
    };

    view! {
        <>
            <Nav/>
            <div>
                <form
                    class="max-w-5xl mx-auto max-md:mx-10"
                    id="add-profile-submit-from"
                    on:submit=submit_click
                >

                    <div class="mb-5">
                        <label
                            for="profile-name"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            Your Name
                        </label>
                        <input
                            type="text"
                            id="profile-name"
                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                            required
                            prop:value=move || name()
                            on:input=move |e| set_name(event_target_value(&e))
                        />
                    </div>
                    <div class="mb-5">
                        <label
                            for="profile-details"
                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                        >
                            Profile Details
                        </label>
                        <MarkdownField
                            set_markdown=set_markdown
                            name=String::from("profile-details")
                            class=String::from(
                                "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                            )
                        />

                        <div class="mb-5">
                            <label
                                for="profile-video"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Profile Video
                            </label>
                            <FileUpload
                                set_cid_props=set_cid
                                accept_file_type=String::from("video/mp4")
                            />
                        </div>

                        <button
                            type="submit"
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >

                            Submit
                        </button>

                    </div>
                </form>
            </div>
        </>
    }
}
