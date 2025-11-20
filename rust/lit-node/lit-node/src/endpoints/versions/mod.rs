pub const LATEST_VERSION: &str = "v2";

pub mod initial;
pub mod v1;
pub mod v2;
pub fn deprecated_endpoint_error() -> rocket::response::status::Custom<serde_json::Value> {
    use lit_api_core::error::ApiError;
    let msg = format!(
        "This endpoint has been deprecated.  Please use the latest SDK with the {} endpoint.",
        LATEST_VERSION
    );
    crate::error::generic_err_code(
        msg,
        crate::error::EC::OldEndpointVersionNoLongerSupported,
        None,
    )
    .handle()
}
