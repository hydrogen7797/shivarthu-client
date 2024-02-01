use crate::components::api::ipfs_request::ipfs_call;
use crate::components::api::select_ipfs_provider::DEFAULT_IPFS_PROVIDER;
use crate::constants::constant::DEFAULT_IPFS_FETCH_PROVIDER;
use crate::services::error::ErrorString;
use icondata;
use leptos::ev::{DragEvent, Event};
use leptos::*;
use leptos_icons::*;
use web_sys::{File, HtmlInputElement};

async fn get_cid_ipfs(
    file_data: Option<File>,
    set_cid: WriteSignal<String>,
    set_cid_props: WriteSignal<String>,
    accept_file_type: String,
    set_spinner: WriteSignal<bool>,
) -> Result<String, ErrorString> {
    if let Some(file) = file_data.clone() {
        let file_type = file.type_();
        let file_name = file.name();
        gloo::console::log!(format! {"{:?}", file_name});
        if file_type == accept_file_type {
            let ipfs_cid = ipfs_call(DEFAULT_IPFS_PROVIDER, file, file_name).await;
            gloo::console::log!(format! {"{:?}", ipfs_cid});
            set_spinner(false);
            set_cid(ipfs_cid.clone());
            set_cid_props(ipfs_cid.clone());
            Ok(ipfs_cid)
        } else {
            Err(ErrorString("Unsupported file type".to_string()))
        }
    } else {
        Err(ErrorString("No file data provided".to_string()))
    }
}

#[component]
pub fn FileUpload(
    #[prop(into)] set_cid_props: WriteSignal<String>,
    accept_file_type: String,
) -> impl IntoView {
    let (cid, set_cid) = create_signal(String::from(""));
    let (spinner, set_spinner) = create_signal(false);
    let (file_option, set_file_option) = create_signal(None);
    let accept_file_type_clone = accept_file_type.clone();
    let async_ipfs = create_resource(
        move || {
            (
                file_option(),
                set_cid,
                set_cid_props,
                accept_file_type_clone.clone(),
                set_spinner,
            )
        },
        |(file_data, set_cid, set_cid_props, accept_file_type, set_spinner)| async move {
            get_cid_ipfs(
                file_data,
                set_cid,
                set_cid_props,
                accept_file_type,
                set_spinner,
            )
            .await
        },
    );
    let file_handle = move |e: Event| {
        set_spinner(true);
        let input: HtmlInputElement = event_target(&e);
        let file_data = input.files().unwrap().get(0);
        gloo::console::log!(format! {"{:?}", file_data});
        set_file_option(file_data);
        let cid = async_ipfs.get().unwrap();
    };
    let ondrop = move |e: DragEvent| {
        gloo::console::log!(format! {"inside drag"});
        set_spinner(true);

        e.prevent_default();
        let file_data: Option<web_sys::File> = e.data_transfer().unwrap().files().unwrap().get(0);
        gloo::console::log!(format! {"{:?}", file_data});
        set_file_option(file_data);
        let cid = async_ipfs.get().unwrap();
    };

    let ondragover = move |e: DragEvent| {
        e.prevent_default();
    };

    let ondragenter = move |e: DragEvent| {
        e.prevent_default();
    };

    view! {
        <div>
            <div class="mb-6">
                <div
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full h-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 min-h-80 flex justify-center items-center"
                    id="drag-container"
                    on:drop=ondrop
                    on:dragover=ondragover
                    on:dragenter=ondragenter
                >
                    <div class="text-center">
                        <div class="mb-4">
                            <Icon
                                icon=icondata::BsCloudUpload
                                width="10em"
                                height="10em"
                                style="color: green"
                                class="inline-block"
                            />
                        </div>
                        <p class="text-lg text-gray-700">
                            {format!(
                                "Drop your {} here or click to select",
                                accept_file_type.clone(),
                            )}

                        </p>
                        <br/>
                        <div>
                            <input
                                id="file-upload"
                                type="file"
                                accept=accept_file_type
                                multiple=false
                                on:input=file_handle
                            />

                        </div>
                    </div>

                </div>
                <div class="flex justify-center items-center">
                    {move || {
                        if spinner() {
                            view! {
                                <div class="mb-4 text-center">
                                    <img src="img/rolling.gif" alt="loading" width="40"/>
                                </div>
                            }
                        } else if !cid().is_empty() {
                            view! {
                                <div>
                                    <video width="320" height="240" controls=true preload="none">
                                        <source
                                            src=format!(
                                                "{}{}",
                                                DEFAULT_IPFS_FETCH_PROVIDER.address,
                                                cid(),
                                            )

                                            type="video/mp4"
                                        />
                                        {"Your browser does not support the video tag."}
                                    </video>
                                </div>
                            }
                        } else {
                            view! { <div></div> }
                        }
                    }}

                </div>
            </div>

        </div>
    }
}
