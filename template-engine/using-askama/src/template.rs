use askama::Template;
use template_engine_core::model;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {
    pub profile: model::ProfileWithServerTime,
}
