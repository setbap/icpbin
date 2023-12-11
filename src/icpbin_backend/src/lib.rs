use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use paste::{IcpPasteError, PasteData, PasteDataCreator, PasteDataUpdater};
use std::cell::RefCell;
use user::{IcpUserError, UserProfile, UserProfileCreator, UserProfileUpdater};

mod paste;
mod user;
type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USERS: RefCell<StableBTreeMap<String, UserProfile, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static PASTES: RefCell<StableBTreeMap<String, PasteData, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );

    static PASTES_SHORT_URL: RefCell<StableBTreeMap<String, String, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );

}

// region: User

fn _get_profile(id: String) -> Option<UserProfile> {
    USERS.with(|service| service.borrow().get(&id))
}

#[ic_cdk::query]
fn get_self_info() -> Result<UserProfile, IcpUserError> {
    let caller: String = ic_cdk::api::caller().to_text();
    let user = _get_profile(caller);
    user.ok_or(IcpUserError::UserNotFound)
}

#[ic_cdk::update]
fn create_new_profile(value: UserProfileCreator) -> Result<UserProfile, IcpUserError> {
    let caller = ic_cdk::api::caller();
    if let Some(_) = _get_profile(caller.to_text()) {
        return Err(IcpUserError::UserAlreadyExist);
    }
    let new_profile = UserProfile::create(caller, value);
    let updated_user = USERS
        .with(|p| p.borrow_mut().insert(new_profile.id.to_text(), new_profile))
        .unwrap();
    Ok(updated_user)
}

#[ic_cdk::update]
fn update_user_profile(value: UserProfileUpdater) -> Result<UserProfile, IcpUserError> {
    let caller = ic_cdk::api::caller();
    if let Some(mut user) = _get_profile(caller.to_text()) {
        user.update(value);
        return Ok(user);
    }
    return Err(IcpUserError::UserNotFound);
}

// endregion: User

// region: Paste

fn _get_paste_by_id(id: String) -> Option<PasteData> {
    PASTES.with(|service| service.borrow().get(&id))
}

fn _get_pastes_from_vec(ids: Vec<String>) -> Option<Vec<PasteData>> {
    let mut pastes = vec![];
    for idx in ids {
        let paste = _get_paste_by_id(idx);
        if let None = paste {
            return None;
        }
        pastes.push(paste.unwrap());
    }
    Some(pastes)
}

fn _get_file_extension(file_name: &str) -> Option<&str> {
    if let Some(dot_position) = file_name.rfind('.') {
        Some(&file_name[dot_position + 1..])
    } else {
        None
    }
}

fn _is_short_url_exist(short_url: &String) -> bool {
    PASTES_SHORT_URL.with(|service| service.borrow().contains_key(short_url))
}

#[ic_cdk::query]
fn get_paste_by_index(index: String) -> Result<PasteData, IcpPasteError> {
    let paste = _get_paste_by_id(index);
    paste.ok_or(IcpPasteError::PasteNotFound)
}

#[ic_cdk::query]
fn get_paste_by_user(caller: Option<Principal>) -> Result<Vec<PasteData>, IcpPasteError> {
    let id = if None == caller {
        ic_cdk::caller()
    } else {
        caller.unwrap()
    };
    let user = _get_profile(id.to_text());

    if let None = user {
        return Err(IcpPasteError::PasteNotFound);
    }
    let user = user.unwrap();
    let pastes = _get_pastes_from_vec(user.paste_indexs);

    pastes.ok_or(IcpPasteError::PasteNotFound)
}

#[ic_cdk::query]
fn get_last_n_paste(count: Option<u8>) -> Result<Vec<PasteData>, IcpPasteError> {
    let mut pastes = vec![];
    let mut ids = vec![];
    let mut count = if None == count { 10 } else { count.unwrap() } as u64;
    if count > 10 {
        count = 10;
    }
    PASTES.with(|p| {
        for (k, _) in p.borrow().iter() {
            ids.push(k.to_string());
        }
    });
    let ids = ids.into_iter().rev();
    for idx in ids {
        let paste = _get_paste_by_id(idx);
        if let None = paste {
            return Err(IcpPasteError::PasteNotFound);
        }
        pastes.push(paste.unwrap());
        if pastes.len() as u64 >= count {
            break;
        }
    }
    Ok(pastes)
}

#[ic_cdk::query]
fn find_paste_by_tag(tag: String) -> Result<Vec<PasteData>, IcpPasteError> {
    let mut pastes = vec![];
    PASTES.with(|p| {
        for (_, v) in p.borrow().iter() {
            if v.tags.contains(&tag) {
                pastes.push(v.clone());
            }
        }
    });
    Ok(pastes)
}

#[ic_cdk::query]
fn find_paste_by_extension(extension: String) -> Result<Vec<PasteData>, IcpPasteError> {
    let mut pastes = vec![];
    PASTES.with(|p| {
        for (_, v) in p.borrow().iter() {
            if let Some(file_extension) = _get_file_extension(&v.name) {
                if file_extension == extension {
                    pastes.push(v.clone());
                }
            }
        }
    });
    Ok(pastes)
}

#[ic_cdk::query]
fn find_paste_by_name(name: String) -> Result<Vec<PasteData>, IcpPasteError> {
    let mut pastes = vec![];
    PASTES.with(|p| {
        for (_, v) in p.borrow().iter() {
            if v.name == name {
                pastes.push(v.clone());
            }
        }
    });
    Ok(pastes)
}

#[ic_cdk::query]
fn find_paste_by_short_url(short_url: String) -> Result<PasteData, IcpPasteError> {
    let paste_id = PASTES_SHORT_URL
        .with(|p| p.borrow().get(&short_url))
        .unwrap();
    let paste = _get_paste_by_id(paste_id.clone());
    paste.ok_or(IcpPasteError::PasteNotFound)
}

#[ic_cdk::update]
fn create_new_paste(value: PasteDataCreator) -> Result<PasteData, IcpPasteError> {
    let short_url = value.short_url.clone();
    let short = short_url.clone().unwrap_or("".to_string());
    if short_url.is_some() && (short.len() < 4 || short.len() > 10) {
        return Err(IcpPasteError::InValidShortURL);
    }
    if short_url.is_some() && _is_short_url_exist(&short) {
        return Err(IcpPasteError::ShortUrlAlreadyExist);
    }
    let caller = ic_cdk::api::caller();
    let user = _get_profile(caller.to_text());
    let is_user_anon = user.is_none();
    let user_id = if !is_user_anon {
        Some(user.clone().unwrap().id.clone())
    } else {
        None
    };
    let new_paste = PasteData::create(user_id, value);
    let new_paste_id = new_paste.id.to_string();
    let updated_user = PASTES
        .with(|p| p.borrow_mut().insert(new_paste_id.clone(), new_paste))
        .unwrap();

    if short_url.is_some() {
        PASTES_SHORT_URL
            .with(|p| {
                p.borrow_mut()
                    .insert(short_url.unwrap(), new_paste_id.clone())
            })
            .unwrap();
    }

    if !is_user_anon {
        let mut user = user.unwrap();
        user.add_new_paste(new_paste_id);
        USERS
            .with(|p| p.borrow_mut().insert(user.id.to_text(), user))
            .unwrap();
    }
    Ok(updated_user)
}

#[ic_cdk::update]
fn update_paste(paste_id: String, value: PasteDataUpdater) -> Result<PasteData, IcpPasteError> {
    let caller = ic_cdk::api::caller();
    let user = _get_profile(caller.to_text());
    let is_user_none = user.is_none();
    if is_user_none {
        return Err(IcpPasteError::PasteIsNotAccessable);
    }
    let paste = _get_paste_by_id(paste_id);
    let is_paste_none = paste.is_none();
    if is_paste_none {
        return Err(IcpPasteError::PasteNotFound);
    }
    let mut paste = paste.unwrap();
    paste.update(value);
    let updated_paste = PASTES
        .with(|p| p.borrow_mut().insert(paste.id.to_string(), paste))
        .unwrap();
    Ok(updated_paste)
}

// endregion: Paste
