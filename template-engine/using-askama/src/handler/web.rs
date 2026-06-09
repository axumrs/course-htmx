use std::sync::LazyLock;

use askama::Template;
use axum::response::Html;
use chrono::Utc;
use template_engine_core::{Result, model};

use crate::template;

static PROFILE: LazyLock<model::Profile> =
    LazyLock::new(|| model::Profile::new("AXUM中文网", "team@mail.axum.eu.org"));

pub async fn index() -> Result<Html<String>> {
    let tpl = template::IndexTemplate {};
    let html = tpl.render()?;
    Ok(Html(html))
}

pub async fn profile_api() -> Result<Html<String>> {
    let profile = get_user_profile().await?;
    let tpl = template::ProfileTemplate { profile };
    let html = tpl.render()?;
    Ok(Html(html))
}

async fn get_user_profile() -> Result<model::ProfileWithServerTime> {
    Ok(model::ProfileWithServerTime {
        profile: (*PROFILE).clone(),
        server_time: Utc::now(),
    })
}
