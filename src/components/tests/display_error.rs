use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum NumberError {
    #[error("Number getter than 20")]
    NumberGreate20,
    #[error("Error parsing to number: `{0}`")]
    ParseIntError(String),
}

#[component]
pub fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| {
        let result_value = event_target_value(&ev).parse::<i32>();
        let result = match result_value {
            Ok(parsed_value) => {
                if parsed_value > 20 {
                    Err(NumberError::NumberGreate20)
                } else {
                    Ok(parsed_value)
                }
            }
            Err(value) => Err(NumberError::ParseIntError(value.to_string())),
        };
        set_value(result)
    };

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number less than than 20 (or something that's not a number!)"
            <input type="number" on:input={on_input}/>
            // the fallback receives a signal containing current errors
            <ErrorBoundary fallback={|errors| {
                view! {
                    <div class="error">
                        <p>" Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}

                        </ul>
                    </div>
                }
            }}>

                {move || value.get().map(|_| ())}
            </ErrorBoundary>
        </label>
    }
}
