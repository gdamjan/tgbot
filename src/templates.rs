use anyhow::Result;
use minijinja::{path_loader, Environment};
use serde::Serialize;
use spin_sdk::http::Response;

pub(crate) fn create_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader("./templates"));
    env
}

pub(crate) fn render<S>(name: &str, ctx: S) -> Result<Response>
where
    S: Serialize,
{
    let env = create_env();
    let tmpl = env.get_template(name)?;
    let body = tmpl.render(ctx)?;

    let response = http::Response::builder()
        .status(http::StatusCode::OK)
        .header(http::header::CONTENT_TYPE, "text/html; charset=\"utf-8\"")
        .body(Some(body.into()))?;
    Ok(response)
}
