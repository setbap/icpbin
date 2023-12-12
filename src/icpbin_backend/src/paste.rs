use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

pub const MAX_PASTE_VALUE_SIZE: u32 = 100;

impl Storable for PasteData {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_PASTE_VALUE_SIZE,
        is_fixed_size: false,
    };
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct PasteData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub content: String,
    pub creator: Option<Principal>,
    pub create_date: i64,
    pub update_date: i64,
    pub expire_date: u32,
    pub tags: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct PasteDataCreator {
    pub short_url: Option<String>,
    pub name: String,
    pub description: String,
    pub content: String,
    pub expire_date: u32,
    pub tags: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct PasteDataUpdater {
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub tags: Option<String>,
}

fn _create_tags(input: String) -> Vec<String> {
    input
        .trim()
        .split(" ")
        .filter_map(|v| {
            if v.is_empty() {
                return None;
            }
            return Some(v.to_string());
        })
        .collect::<Vec<String>>()
}

fn _get_now() -> i64 {
    chrono::Utc::now().timestamp_micros()
}

impl PasteData {
    pub fn create(id: Option<Principal>, info: PasteDataCreator) -> Self {
        let now = _get_now();
        PasteData {
            id: Uuid::new_v4().to_string(),
            name: info.name,
            creator: id,
            description: info.description,
            expire_date: info.expire_date,
            content: info.content,
            tags: _create_tags(info.tags),
            create_date: now,
            update_date: now,
        }
    }

    pub fn update(&mut self, info: PasteDataUpdater) {
        if let Some(name) = info.name {
            self.name = name;
        }
        if let Some(desc) = info.description {
            self.content = desc;
        }
        if let Some(content) = info.content {
            self.content = content;
        }

        if let Some(tags) = info.tags {
            self.tags = _create_tags(tags);
        }

        self.update_date = _get_now();
    }

    pub fn clear(&mut self) {
        self.name = "__DELETED__".to_string();
        self.content = "__DELETED__".to_string();
        self.tags = Vec::new();
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum IcpPasteError {
    InValidShortURL,
    ShortUrlAlreadyExist,
    PasteNotFound,
    PasteAlreadyExist,
    PasteIsNotAccessable,
    WrongExpireDate,
}
