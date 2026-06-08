use axum::response::Html;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub title: String,
}

pub async fn message_list() -> Html<String> {
    let data = message_list_data().await;
    let data = data
        .iter()
        .map(|m| format!("<div>{}</div>", m.title))
        .collect::<Vec<_>>()
        .join("\n");
    Html(data)
}

async fn message_list_data() -> Vec<Message> {
    (0..10)
        .map(|i| Message {
            id: Uuid::now_v7(),
            title: format!("消息 #{}", i),
        })
        .collect()
}
