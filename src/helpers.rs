use anyhow::Result;
use spin_sdk::http::{Params, Request, Response};

pub(crate) fn redirect(location: &str) -> Result<Response> {
    let res = http::Response::builder()
        .status(http::StatusCode::SEE_OTHER)
        .header(http::header::LOCATION, location)
        .body(None)?;
    Ok(res)
}

// pub(crate) fn not_implemented() -> Result<Response> {
//     let body = "not implemented";
//     let response = http::Response::builder()
//         .status(http::StatusCode::NOT_IMPLEMENTED)
//         .body(Some(body.into()))?;
//     Ok(response)
// }

pub(crate) fn no_content(_req: Request, _params: Params) -> Result<Response> {
    let response = http::Response::builder()
        .status(http::StatusCode::NO_CONTENT)
        .body(None)?;
    Ok(response)
}

pub(crate) fn unauthorized() -> Result<Response> {
    let body = "authorization required";
    let response = http::Response::builder()
        .header(
            http::header::WWW_AUTHENTICATE,
            "Basic realm=\"Authorization Required\"",
        )
        .status(http::StatusCode::UNAUTHORIZED)
        .body(Some(body.into()))?;
    Ok(response)
}
