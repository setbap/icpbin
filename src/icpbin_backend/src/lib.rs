use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use user::{IcpUserError, UserProfile, UserProfileCreator, UserProfileUpdater};

mod paste;
type Memory = VirtualMemory<DefaultMemoryImpl>;

// For a type to be used in a `StableBTreeMap`, it needs to implement the `Storable`
// trait, which specifies how the type can be serialized/deserialized.
//
// In this example, we're using candid to serialize/deserialize the struct, but you
// can use anything as long as you're maintaining backward-compatibility. The
// backward-compatibility allows you to change your struct over time (e.g. adding
// new fields).
//
// The `Storable` trait is already implemented for several common types (e.g. u64),
// so you can use those directly without implementing the `Storable` trait for them.

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USERS: RefCell<StableBTreeMap<String, UserProfile, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
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
//
