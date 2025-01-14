use leptos::prelude::*;
use crate::constants::constant::NODE_URL;
use leptos::ev::SubmitEvent;
use serde::{Deserialize, Serialize};
use leptos::task::spawn_local;
use leptos::html;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos_router::hooks::use_params_map;
use crate::components::schelling_game::positive_externality::views::view_post_positive_externality::ViewPostPositiveExternality;
use crate::components::navigation::nav::Nav;


#[derive(Serialize, Deserialize, Clone)]
struct PaginatedPosts {
    posts: Vec<u64>,
    current_page: u64,
    total_pages: u64,
}

#[component]
pub fn ViewPositiveExternalityLatest() -> impl IntoView {
    let (page, set_page) = signal(1);
    let (page_size, set_page_size) = signal(10);
    let (posts, set_posts) = signal::<Option<Vec<u64>>>(None);
    let (total_posts_length, set_total_posts_length) = signal(0);
    let (total_pages, set_total_pages) = signal(0);
    let input_element_page: NodeRef<html::Input> = NodeRef::new();

    let input_element_page_size : NodeRef<html::Input> = NodeRef::new();

    let params = use_params_map();
    
    let user = move || {
        params.with(|params| {
            params
                .get("user")
                .unwrap_or_default()
        })
    };

    let user_value = untrack( || user());

    gloo::console::log!(user_value);

    // Fetch paginated posts when `page` or `page_size` changes
    Effect::new(move |_| {
        let user = user(); // Replace with actual user ID
        let page = page();
        let page_size = page_size();

        spawn_local(async move {
            let result = paginate_posts_by_address(user, page, page_size).await;
            match result {
                Ok((Some(posts), total_length)) => {
                    set_posts.set(Some(posts.clone()));
                    set_total_posts_length.set(total_length);
                    set_total_pages.set((total_length + page_size - 1) / page_size);
                }
                _ => {
                    set_posts.set(None);
                    set_total_posts_length.set(0);
                    set_total_pages.set(0);
                }
            }
        });
    });

    // Handle page navigation
    let go_to_page = move |new_page: u64| {
        if new_page > 0 && new_page <= total_pages() {
            set_page.set(new_page);
        }
    };

    // Handle page size change
    let update_page = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input = input_element_page.get().expect("<input> should be mounted").value();
        gloo::console::log!(input.clone());
        let new_page = input.parse::<u64>().unwrap_or(1);
        set_page.set(new_page); // Reset to the first page
    };

        // Handle page size change
        let update_page_size = move |ev: SubmitEvent| {
            ev.prevent_default();
            let input = input_element_page_size.get().expect("<input> should be mounted").value();
            let new_size = input.parse::<u64>().unwrap_or(10);
            set_page_size.set(new_size);
            set_page.set(1); // Reset to the first page
        };

    view! {
        <>
            <Nav/>
            <div class="p-4 space-y-4">
                <h1 class="text-2xl font-bold">Paginated Posts</h1>

                // Display posts
                <div class="space-y-2">
                    {move || match posts() {
                        Some(posts) => {
                            posts
                                .into_iter()
                                .map(|post| {
                                    view! {
                                        <div class="p-2 border rounded">
                                            <ViewPostPositiveExternality id=post/>
                                        </div>
                                    }
                                })
                                .collect_view()
                                .into_any()
                        }
                        None => view! { <div>No posts found.</div> }.into_any(),
                    }}

                </div>

                // Pagination controls
                <div class="flex items-center justify-between">
                    <button
                        class="px-4 py-2 bg-blue-500 text-white rounded disabled:opacity-50"
                        on:click=move |_| go_to_page(page() - 1)
                        disabled=move || page() <= 1
                    >
                        "Previous"
                    </button>
                    <span class="text-gray-700">
                        "Page " {page} " of " {total_pages} " (Total Posts: " {total_posts_length}
                        ")"
                    </span>
                    <button
                        class="px-4 py-2 bg-blue-500 text-white rounded disabled:opacity-50"
                        on:click=move |_| { go_to_page(page() + 1) }
                        disabled=move || { page() >= total_pages() }
                    >
                        "Next"
                    </button>
                </div>

                // Page selector
                <form on:submit=update_page class="flex items-center space-x-2">
                    <label class="text-gray-700">Page Number:</label>
                    <input
                        type="number"
                        class="p-2 border rounded"
                        node_ref=input_element_page
                        value=move || { page().to_string() }
                    />
                    <button type="submit" class="px-4 py-2 bg-green-500 text-white rounded">
                        "Update"
                    </button>
                </form>
                <br/>

                // Page size selector
                <form on:submit=update_page_size class="flex items-center space-x-2">
                    <label class="text-gray-700">Page Size:</label>
                    <input
                        type="number"
                        class="p-2 border rounded"
                        node_ref=input_element_page_size
                        value=move || { page_size().to_string() }
                    />
                    <button type="submit" class="px-4 py-2 bg-green-500 text-white rounded">
                        Update
                    </button>
                </form>
            </div>
        </>
    }
}

// Mock API function (replace with your actual API call)
async fn paginate_posts_by_address(
    user: String,
    page: u64,
    page_size: u64,
) -> Result<(Option<Vec<u64>>, u64), String> {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let all_posts_length: u64 = client
        .request(
            "positiveexternality_postbyaddresslength",
            rpc_params![user.clone()],
        )
        .await
        .unwrap();

        gloo::console::log!("allposte", all_posts_length.clone());


        let posts: Option<Vec<u64>> = client
        .request(
            "positiveexternality_paginateposts_latest",
            rpc_params![user, page, page_size],
        )
        .await
        .unwrap();

        gloo::console::log!("posts", posts.clone());



    Ok((posts, all_posts_length))
}