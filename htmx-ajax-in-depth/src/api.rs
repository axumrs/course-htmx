use axum::{Form, response::Html};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub title: String,
}
impl Topic {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Uuid::now_v7(),
            title: title.into(),
        }
    }
}

#[derive(Deserialize)]
pub struct TopicSearchForm {
    pub keyword: Option<String>,
}

pub async fn topic_list(Form(form): Form<TopicSearchForm>) -> Html<String> {
    let ls = get_topics(form.keyword).await;
    let html = ls
        .iter()
        .map(|t| format!("<li>{}</li>", t.title))
        .collect::<Vec<_>>()
        .join("");
    Html(html)
}

async fn get_topics(keyword: Option<String>) -> Vec<Topic> {
    let data = vec![
        Topic::new("站内通知：SSE 状态共享及数据库访问"),
        Topic::new("在axum中获取请求数据"),
        Topic::new("获取访问令牌及调用API"),
        Topic::new("将静态资源嵌入二进制文件"),
        Topic::new("axum的状态共享"),
    ];

    if let Some(keyword) = &keyword {
        return data
            .into_iter()
            .filter(|topic| {
                topic
                    .title
                    .to_lowercase()
                    .contains(keyword.to_lowercase().as_str())
            })
            .collect();
    }
    data
}
