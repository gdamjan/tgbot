use anyhow::Result;

use spin_sdk::{
    http::{HeaderValue, IntoResponse, Params, Request, Response, Router},
    http_component, variables,
};

use helpers::no_content;

mod helpers;
mod templates;

#[http_component]
fn handler(req: Request) -> Result<impl IntoResponse> {
    let mut router = Router::new();

    router.get("/", index);
    router.post("/webhook", webhook);
    router.get_async("/me", |_r: Request, _p: Params| proxy("Me"));
    router.get_async("/hookinfo", |_r: Request, _p: Params| proxy("WebhookInfo"));
    router.post_async("/start", |_r: Request, _p: Params| start());
    router.post_async("/stop", |_r: Request, _p: Params| stop());
    router.get("/favicon.ico", |_r: Request, _p: Params| no_content());

    Ok(router.handle(req))
}

fn index(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    match variables::get("telegram_token") {
        Ok(_) => templates::render("index.html", ()),
        Err(_) => templates::render("readme.html", ()),
    }
}

/// What?
async fn proxy(s: &str) -> Result<impl IntoResponse> {
    let api_endpoint = variables::get("telegram_api_endpoint").expect("could not get variable");

    let url = format!("{api_endpoint}/get{s}");
    let request = Request::get(url).body(()).build();
    let response: Response = spin_sdk::http::send(request).await?;

    // let content_type = response
    //     .header("content-type")
    //     .unwrap_or(&HeaderValue::string("application/json".into()));

    // response.status()
    // let response = Response::builder()
    //     .status(response.status().clone())
    //     .header("content-type", content_type.as_str())
    //     .body(res.body())
    //     .build();
    Ok(response)
}

fn webhook(req: Request, params: Params) -> Result<impl IntoResponse> {
    // https://core.telegram.org/bots/webhooks
    let _ = (req, params);
    no_content()
}

async fn start() -> Result<impl IntoResponse> {
    let api_endpoint = variables::get("telegram_api_endpoint").expect("could not get variable");

    let url = format!("{api_endpoint}/setWebhook");
    let request = Request::post(url, ()).build();

    let _res: Response = spin_sdk::http::send(request).await?;

    no_content()
}

async fn stop() -> Result<impl IntoResponse> {
    let api_endpoint = variables::get("telegram_api_endpoint").expect("could not get variable");

    let url = format!("{api_endpoint}/deleteWebhook");
    let request = Request::post(url, ()).build();
    let _res: Response = spin_sdk::http::send(request).await?;
    no_content()
}

/// check telegram_token is sent as bearer authorization header
/// and is the same as configured
fn authorized(req: &Request) -> bool {
    let telegram_token = variables::get("telegram_token");
    if telegram_token.is_err() {
        return false;
    }
    let telegram_token = telegram_token.unwrap();

    let expected = HeaderValue::string(format!("Bearer {telegram_token}"));
    match req.header("Authorization") {
        Some(v) => v == &expected,
        None => false,
    }
}
