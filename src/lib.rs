#![no_std]

//! # Registry Contract
//!
//! A simple key/value registry for Soroban with:
//! - Explicit access control (admin/owner role).
//! - Bounded input sizes and entry counts.
//! - Events emitted for every state change.
//!
//! ## Access control model
//!
//! - **Admin**: the account set at initialization. The admin is the only
//!   account allowed to `create_entry`, `update_entry`, and `remove_entry`.
//! - **Anyone**: may read entries via `get_entry` / `entry_count`.
//!
//! Unauthorized callers are rejected with [`Error::Unauthorized`].
//!
//! ## Immutability
//!
//! This contract is intentionally **immutable**: there is no upgrade function.
//! See the ADR (`docs/adr/0001-immutable-vs-upgradeable.md`) for the rationale.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Bytes, Env, Map,
    Symbol,
};

/// Maximum length (in bytes) of an entry key.
pub const MAX_KEY_LEN: u32 = 64;
/// Maximum length (in bytes) of an entry value.
pub const MAX_VALUE_LEN: u32 = 1024;
/// Maximum number of entries that may be stored at once.
pub const MAX_ENTRIES: u32 = 100;

/// Storage keys used by the contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// The admin address allowed to mutate the registry.
    Admin,
    /// The map of entry key -> value.
    Entries,
}

/// Custom errors for validation and authorization failures.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Caller is not the admin.
    Unauthorized = 1,
    /// Key is empty.
    EmptyInput = 2,
    /// Key exceeds [`MAX_KEY_LEN`].
    KeyTooLong = 3,
    /// Value exceeds [`MAX_VALUE_LEN`].
    ValueTooLong = 4,
    /// Registry already holds [`MAX_ENTRIES`] entries.
    TooManyEntries = 5,
    /// Entry does not exist.
    NotFound = 6,
    /// Entry already exists.
    AlreadyExists = 7,
    /// Contract has not been initialized.
    NotInitialized = 8,
    /// Contract has already been initialized.
    AlreadyInitialized = 9,
}

/// Event topics.
const TOPIC_CREATE: Symbol = symbol_short!("create");
const TOPIC_UPDATE: Symbol = symbol_short!("update");
const TOPIC_REMOVE: Symbol = symbol_short!("remove");

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    /// Initialize the contract with an admin address.
    ///
    /// Can only be called once. Emits an `init` event with the admin as an
    /// indexed topic.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::Entries, &Map::<Bytes, Bytes>::new(&env));

        // Event: topics (init, admin), data (admin)
        env.events()
            .publish((symbol_short!("init"), admin.clone()), admin);

        Ok(())
    }

    /// Create a new entry.
    ///
    /// - Requires auth from the admin.
    /// - Reverts with [`Error::EmptyInput`] if `key` is empty.
    /// - Reverts with [`Error::KeyTooLong`] if `key` exceeds [`MAX_KEY_LEN`].
    /// - Reverts with [`Error::ValueTooLong`] if `value` exceeds [`MAX_VALUE_LEN`].
    /// - Reverts with [`Error::TooManyEntries`] if the registry is full.
    /// - Reverts with [`Error::AlreadyExists`] if the key already exists.
    ///
    /// Emits a `create` event with topics `(create, key, caller)` and data
    /// `(key, value)`.
    pub fn create_entry(
        env: Env,
        caller: Address,
        key: Bytes,
        value: Bytes,
    ) -> Result<(), Error> {
        Self::require_admin(&env, &caller)?;
        Self::validate_key(&key)?;
        Self::validate_value(&value)?;

        let mut entries = Self::load_entries(&env);

        if entries.contains_key(key.clone()) {
            return Err(Error::AlreadyExists);
        }
        if entries.len() >= MAX_ENTRIES {
            return Err(Error::TooManyEntries);
        }

        entries.set(key.clone(), value.clone());
        Self::save_entries(&env, &entries);

        // Event: topics (create, key, caller), data (key, value)
        env.events().publish(
            (TOPIC_CREATE, key.clone(), caller.clone()),
            (key, value),
        );

        Ok(())
    }

    /// Update an existing entry.
    ///
    /// - Requires auth from the admin.
    /// - Reverts with [`Error::EmptyInput`] if `key` is empty.
    /// - Reverts with [`Error::KeyTooLong`] if `key` exceeds [`MAX_KEY_LEN`].
    /// - Reverts with [`Error::ValueTooLong`] if `value` exceeds [`MAX_VALUE_LEN`].
    /// - Reverts with [`Error::NotFound`] if the key does not exist.
    ///
    /// Emits an `update` event with topics `(update, key, caller)` and data
    /// `(key, old_value, new_value)`.
    pub fn update_entry(
        env: Env,
        caller: Address,
        key: Bytes,
        value: Bytes,
    ) -> Result<(), Error> {
        Self::require_admin(&env, &caller)?;
        Self::validate_key(&key)?;
        Self::validate_value(&value)?;

        let mut entries = Self::load_entries(&env);

        let old_value = match entries.get(key.clone()) {
            Some(v) => v,
            None => return Err(Error::NotFound),
        };

        entries.set(key.clone(), value.clone());
        Self::save_entries(&env, &entries);

        // Event: topics (update, key, caller), data (key, old_value, new_value)
        env.events().publish(
            (TOPIC_UPDATE, key.clone(), caller.clone()),
            (key, old_value, value),
        );

        Ok(())
    }

    /// Remove an existing entry.
    ///
    /// - Requires auth from the admin.
    /// - Reverts with [`Error::EmptyInput`] if `key` is empty.
    /// - Reverts with [`Error::KeyTooLong`] if `key` exceeds [`MAX_KEY_LEN`].
    /// - Reverts with [`Error::NotFound`] if the key does not exist.
    ///
    /// Emits a `remove` event with topics `(remove, key, caller)` and data
    /// `(key, old_value)`.
    pub fn remove_entry(env: Env, caller: Address, key: Bytes) -> Result<(), Error> {
        Self::require_admin(&env, &caller)?;
        Self::validate_key(&key)?;

        let mut entries = Self::load_entries(&env);

        let old_value = match entries.get(key.clone()) {
            Some(v) => v,
            None => return Err(Error::NotFound),
        };

        entries.remove(key.clone());
        Self::save_entries(&env, &entries);

        // Event: topics (remove, key, caller), data (key, old_value)
        env.events()
            .publish((TOPIC_REMOVE, key.clone(), caller.clone()), (key, old_value));

        Ok(())
    }

    /// Read an entry. Returns `None` if it does not exist.
    pub fn get_entry(env: Env, key: Bytes) -> Option<Bytes> {
        Self::load_entries(&env).get(key)
    }

    /// Return the number of stored entries.
    pub fn entry_count(env: Env) -> u32 {
        Self::load_entries(&env).len()
    }

    /// Return the current admin address, if initialized.
    pub fn admin(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Admin)
    }

    // ---------------------------------------------------------------------
    // Internal helpers
    // ---------------------------------------------------------------------

    fn require_admin(env: &Env, caller: &Address) -> Result<(), Error> {
        caller.require_auth();

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;

        if &admin != caller {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    fn validate_key(key: &Bytes) -> Result<(), Error> {
        if key.is_empty() {
            return Err(Error::EmptyInput);
        }
        if key.len() > MAX_KEY_LEN {
            return Err(Error::KeyTooLong);
        }
        Ok(())
    }

    fn validate_value(value: &Bytes) -> Result<(), Error> {
        if value.len() > MAX_VALUE_LEN {
            return Err(Error::ValueTooLong);
        }
        Ok(())
    }

    fn load_entries(env: &Env) -> Map<Bytes, Bytes> {
        env.storage()
            .instance()
            .get(&DataKey::Entries)
            .unwrap_or_else(|| Map::new(env))
    }

    fn save_entries(env: &Env, entries: &Map<Bytes, Bytes>) {
        env.storage().instance().set(&DataKey::Entries, entries);
    }
}

#[cfg(test)]
mod test;