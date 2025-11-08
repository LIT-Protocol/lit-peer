use leptos::prelude::*;

#[component]
pub fn AddressRenderer<F>(
    class: String,
    #[prop(into)] value: Signal<String>,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    on_change: F,
    #[allow(unused_variables)] // onchange & index need to be part of the signature for now
    index: usize,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let copy_function = format!(
        "javascript: navigator.clipboard.writeText('{}')",
        value.get()
    );
    let address = value.get();
    let address = match address.len() {
        42 => format!("{}..{}", &address[..6], &address[38..]),
        _ => address,
    };
    view! {
        <td class=class>
            <a href={copy_function}>{address}</a>
        </td>
    }
}
