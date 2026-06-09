use std::ops::Deref;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl Profile {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: Uuid::now_v7(),
            name: name.into(),
            email: email.into(),
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileWithServerTime {
    #[serde(flatten)]
    pub profile: Profile,
    pub server_time: chrono::DateTime<chrono::Utc>,
}

impl Deref for ProfileWithServerTime {
    type Target = Profile;
    fn deref(&self) -> &Self::Target {
        &self.profile
    }
}
