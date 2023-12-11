use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const MAX_USER_VALUE_SIZE: u32 = 100;

impl Storable for UserProfile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_USER_VALUE_SIZE,
        is_fixed_size: false,
    };
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: Principal,
    pub name: String,
    pub gravatar: String,
    pub bio: String,
    pub paste_indexs: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct UserProfileCreator {
    name: String,
    gravatar: String,
    bio: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct UserProfileUpdater {
    name: Option<String>,
    gravatar: Option<String>,
    bio: Option<String>,
}

impl UserProfile {
    pub fn create(id: Principal, info: UserProfileCreator) -> Self {
        UserProfile {
            id,
            name: info.name,
            gravatar: info.gravatar,
            bio: info.bio,
            paste_indexs: Vec::new(),
        }
    }

    pub fn update(&mut self, info: UserProfileUpdater) {
        if let Some(name) = info.name {
            self.name = name;
        }
        if let Some(bio) = info.bio {
            self.bio = bio;
        }
        if let Some(gravatar) = info.gravatar {
            self.gravatar = gravatar;
        }
    }

    pub fn add_new_paste(&mut self, paste_index: String) {
        self.paste_indexs.push(paste_index);
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum IcpUserError {
    UserAlreadyExist,
    UserNotFound,
}
