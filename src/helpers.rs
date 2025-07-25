use anyhow::Result;
use spin_sdk::http::Response;

pub(crate) fn redirect(location: &str) -> Result<Response> {
    let response = Response::builder()
        .status(303) // HTTP 303 See Other
        .header("location", location)
        .body(())
        .build();
    Ok(response)
}

pub(crate) fn not_implemented() -> Result<Response> {
    let response = Response::builder()
        .status(501)
        .body("not implemented")
        .build();
    Ok(response)
}

pub(crate) fn no_content() -> Result<Response> {
    // HTTP 204 No Content
    let response = Response::builder().status(204).body(()).build();
    Ok(response)
}

pub(crate) fn unauthorized() -> Result<Response> {
    let response = Response::builder()
        .header("WWW-Authenticate", "Basic realm=\"Authorization Required\"")
        .status(401) // HTTP 401 Unauthorized
        .body("authorization required")
        .build();
    Ok(response)
}
