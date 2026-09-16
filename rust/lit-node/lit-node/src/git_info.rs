// Kept as a tracked file (must exist in the repo), but intentionally STATIC.
// The value is injected at compile time by build.rs via cargo:rustc-env.
// We use `option_env!` (instead of `env!`) so builds that don't run the build script (or don't
// have git metadata available) don't hard-fail compilation; they will report "n/a" instead.
pub const GIT_COMMIT_HASH: &str = match option_env!("GIT_COMMIT_HASH") {
    Some(v) => v,
    None => "n/a",
};
