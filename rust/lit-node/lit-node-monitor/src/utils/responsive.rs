pub fn is_mobile() -> bool {
    use leptos_use::{BreakpointsTailwind, breakpoints_tailwind, use_breakpoints};
    use_breakpoints(breakpoints_tailwind()).is_lt(BreakpointsTailwind::Md)
}

pub fn is_localhost_build() -> bool {
    let href = leptos::prelude::window()
        .location()
        .href()
        .expect("Failed to get location.href");
    href.contains("localhost")
}
