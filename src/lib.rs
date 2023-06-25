use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use helpers::{no_content, redirect};
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    HeaderValue,
};
use spin_sdk::{
    config,
    http::{Params, Request, Response, Router},
    http_component,
    key_value::Store,
};

mod helpers;
mod templates;

const TOKEN_STORE_KEY: &str = "TELEGRAM_BOT_TOKEN";

#[http_component]
fn handler(req: Request) -> Result<Response> {
    let mut router = Router::new();

    router.get("/", index);
    router.get("/favicon.ico", no_content);
    router.get("/me", show_me);
    router.post("/token", update_token);
    router.delete("/token", delete_token);
    router.post("/webhook", webhook);
    router.post("/start", start);
    router.post("/stop", stop);

    if req.uri() == "/webhook" || authorized(&req) {
        router.handle(req)
    } else {
        helpers::unauthorized()
    }
}

fn index(_req: Request, _params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    if store.exists(TOKEN_STORE_KEY)? {
        templates::render("index.html", ())
    } else {
        templates::render("setup.html", ())
    }
}

fn show_me(_req: Request, _params: Params) -> Result<Response> {
    let token = get_stored_token();
    match token {
        Err(err) => show_error(err),
        Ok(token) => {
            let url = format!("https://api.telegram.org/bot{}/getMe", token);
            let request = http::Request::builder().method("GET").uri(url).body(None)?;
            let response = spin_sdk::outbound_http::send_request(request)?;

            let content_type = response
                .headers()
                .get(CONTENT_TYPE)
                .cloned()
                .unwrap_or(HeaderValue::from_static("application/json"));

            let response = http::Response::builder()
                .status(response.status())
                .header(CONTENT_TYPE, content_type)
                .body(response.body().clone())?;
            Ok(response)
        }
    }
}

fn get_stored_token() -> Result<String> {
    let store = Store::open_default()?;
    store
        .get(TOKEN_STORE_KEY)
        .map(|s| String::from_utf8_lossy(&s).to_string())
        .map_err(anyhow::Error::from)
}

fn show_error(err: anyhow::Error) -> Result<Response> {
    let body = format!("{}", err);
    let response = http::Response::builder()
        .status(http::StatusCode::NOT_IMPLEMENTED)
        .header(CONTENT_TYPE, "text/plain")
        .body(Some(body.into()))?;
    Ok(response)
}

fn update_token(req: Request, _params: Params) -> Result<Response> {
    let body = req.body().clone().unwrap();
    let mut post = form_urlencoded::parse(&body);
    if let Some(token) = post
        .find(|item| item.0 == "token")
        .map(|item| item.1.to_string())
        .and_then(|t| if t.is_empty() { None } else { Some(t) })
    {
        let store = Store::open_default()?;
        store.set(TOKEN_STORE_KEY, token)?;
    }
    redirect("/")
}

fn delete_token(req: Request, params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    store.delete(TOKEN_STORE_KEY)?;
    no_content(req, params)
}

fn webhook(_req: Request, _params: Params) -> Result<Response> {
    // https://core.telegram.org/bots/webhooks
    let response = http::Response::builder()
        .status(http::StatusCode::NO_CONTENT)
        .body(None)?;
    Ok(response)
}

fn start(_req: Request, _params: Params) -> Result<Response> {
    // call https://api.telegram.org/bot<YOURTOKEN>/setWebhook
    helpers::redirect("/")
}

fn stop(_req: Request, _params: Params) -> Result<Response> {
    // call https://api.telegram.org/bot<YOURTOKEN>/deleteWebhook
    helpers::redirect("/")
}

/// check username and password via standard authorization
fn authorized(req: &Request) -> bool {
    let username = config::get("username").expect("could not get variable");
    let password = config::get("password").expect("could not get variable");
    let user_pass = format!("{}:{}", username, password);
    let encoded = general_purpose::STANDARD.encode(user_pass);
    let expected = format!("Basic {}", encoded);
    let authorized = req
        .headers()
        .get(AUTHORIZATION)
        .map_or(false, |s| s == expected.as_str());
    authorized
}
