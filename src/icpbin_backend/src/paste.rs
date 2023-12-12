use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const MAX_PASTE_VALUE_SIZE: u32 = 16 * 1024;
pub const DELETE_TEPMLATE: &str = "DELETE";

// allow to store Paste Data in the stable memory
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
    // id is the unique identifier of the paste
    pub id: String,
    pub name: String,
    pub description: String,
    pub content: String,
    // creator is the principal who created the paste and it will be null when the paste is created by not logged in user
    pub creator: Option<Principal>,
    // track number of change in paste
    pub version: i32,
    // time to convert date to delete, it is base on second
    pub expire_date: u32,
    // container to push any extra data about PasteData
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

impl PasteData {
    pub fn create(id: u64, user_id: Option<Principal>, info: PasteDataCreator) -> Self {
        PasteData {
            id: id.to_string(),
            name: info.name,
            creator: user_id,
            description: info.description,
            expire_date: info.expire_date,
            content: info.content,
            tags: _create_tags(info.tags),
            version: 1,
        }
    }

    pub fn update(&mut self, info: PasteDataUpdater) {
        if let Some(name) = info.name {
            self.name = name;
        }
        if let Some(desc) = info.description {
            self.description = desc;
        }
        if let Some(content) = info.content {
            self.content = content;
        }
        if let Some(tags) = info.tags {
            self.tags = _create_tags(tags);
        }

        // increase number of change
        self.version += 1;
    }

    // clear the content of the paste
    pub fn clear(&mut self) {
        self.name = DELETE_TEPMLATE.to_string();
        self.content = DELETE_TEPMLATE.to_string();
        self.tags = Vec::new();
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum IcpPasteError {
    ShortUrlShouldBeBetween4And10,
    ShortUrlAlreadyExist,
    PasteNotFound,
    PasteAlreadyExist,
    PasteIsNotAccessable,
    WrongExpireDate,
}
