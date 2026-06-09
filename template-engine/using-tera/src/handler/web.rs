use std::sync::LazyLock;

use axum::response::Html;
use chrono::Utc;
use template_engine_core::{Result, model};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "templates"]
pub struct Template;

static PROFILE: LazyLock<model::Profile> =
    LazyLock::new(|| model::Profile::new("AXUM中文网", "team@mail.axum.eu.org"));

pub async fn index() -> Result<Html<String>> {
    let tpl_data = Template::get("index.html").unwrap();
    let tpl_data = std::str::from_utf8(&tpl_data.data)?;
    let mut t = tera::Tera::default();
    let ctx = tera::Context::new();
    let html = t.render_str(tpl_data, &ctx)?;
    Ok(Html(html))
}

pub async fn profile_api() -> Result<Html<String>> {
    let tpl_data = Template::get("profile.html").unwrap();
    let tpl_data = std::str::from_utf8(&tpl_data.data)?;
    let mut t = tera::Tera::default();
    let mut ctx = tera::Context::new();
    let profile = get_user_profile().await?;
    ctx.insert("profile", &profile);
    let html = t.render_str(tpl_data, &ctx)?;
    Ok(Html(html))
}

async fn get_user_profile() -> Result<model::ProfileWithServerTime> {
    Ok(model::ProfileWithServerTime {
        profile: (*PROFILE).clone(),
        server_time: Utc::now(),
    })
}
