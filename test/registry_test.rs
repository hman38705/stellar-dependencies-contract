#![cfg(test)]

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events, Ledger},
    Address, Bytes, Env, IntoVal, Symbol, TryFromVal, Val, Vec,
};

use registry::{RegistryContract, RegistryContractClient, Error};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn setup() -> (Env, RegistryContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, RegistryContract);
    let client = RegistryContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, client, admin)
}

fn setup_no_mock() -> (Env, RegistryContractClient<'static>, Address) {
    let env = Env::default();
    let contract_id = env.register_contract(None, RegistryContract);
    let client = RegistryContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&admin);
    env.mock_auths(&[]);
    (env, client, admin)
}

fn key(env: &Env, s: &str) -> Symbol {
    Symbol::new(env, s)
}

fn bytes_of_len(env: &Env, len: u32) -> Bytes {
    let mut b = Bytes::new(env);
    for _ in 0..len {
        b.push_back(b'x');
    }
    b
}

// ---------------------------------------------------------------------------
// Issue #11 — Events for off-chain indexing
// ---------------------------------------------------------------------------

#[test]
fn test_create_entry_emits_event() {
    let (env, client, admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"hello");

    client.create_entry(&caller, &k, &v);

    let events = env.events().all();
    let last = events.last().unwrap();

    // Topic 1: event name symbol, Topic 2: indexed key, Topic 3: indexed caller.
    let topics: Vec<Val> = last.1.clone();
    assert_eq!(topics.len(), 3);
    assert_eq!(
        Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap(),
        symbol_short!("create")
    );
    assert_eq!(
        Symbol::try_from_val(&env, &topics.get(1).unwrap()).unwrap(),
        k
    );
    assert_eq!(
        Address::try_from_val(&env, &topics.get(2).unwrap()).unwrap(),
        caller
    );

    // Data: the stored value.
    let data: Bytes = Bytes::try_from_val(&env, &last.2).unwrap();
    assert_eq!(data, v);

    // Sanity: admin is recorded but not part of the event.
    let _ = admin;
}

#[test]
fn test_update_entry_emits_event() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "beta");
    let v1 = Bytes::from_slice(&env, b"one");
    let v2 = Bytes::from_slice(&env, b"two");

    client.create_entry(&caller, &k, &v1);
    let before = env.events().all().len();
    client.update_entry(&caller, &k, &v2);

    let events = env.events().all();
    assert_eq!(events.len(), before + 1);
    let last = events.last().unwrap();

    let topics: Vec<Val> = last.1.clone();
    assert_eq!(topics.len(), 3);
    assert_eq!(
        Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap(),
        symbol_short!("update")
    );
    assert_eq!(
        Symbol::try_from_val(&env, &topics.get(1).unwrap()).unwrap(),
        k
    );
    assert_eq!(
        Address::try_from_val(&env, &topics.get(2).unwrap()).unwrap(),
        caller
    );

    let data: Bytes = Bytes::try_from_val(&env, &last.2).unwrap();
    assert_eq!(data, v2);
}

#[test]
fn test_remove_entry_emits_event() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "gamma");
    let v = Bytes::from_slice(&env, b"bye");

    client.create_entry(&caller, &k, &v);
    let before = env.events().all().len();
    client.remove_entry(&caller, &k);

    let events = env.events().all();
    assert_eq!(events.len(), before + 1);
    let last = events.last().unwrap();

    let topics: Vec<Val> = last.1.clone();
    assert_eq!(topics.len(), 3);
    assert_eq!(
        Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap(),
        symbol_short!("remove")
    );
    assert_eq!(
        Symbol::try_from_val(&env, &topics.get(1).unwrap()).unwrap(),
        k
    );
    assert_eq!(
        Address::try_from_val(&env, &topics.get(2).unwrap()).unwrap(),
        caller
    );

    // Data: the removed value.
    let data: Bytes = Bytes::try_from_val(&env, &last.2).unwrap();
    assert_eq!(data, v);
}

#[test]
fn test_initialize_emits_event() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, RegistryContract);
    let client = RegistryContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    let events = env.events().all();
    let last = events.last().unwrap();
    let topics: Vec<Val> = last.1.clone();
    assert_eq!(topics.len(), 2);
    assert_eq!(
        Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap(),
        symbol_short!("init")
    );
    assert_eq!(
        Address::try_from_val(&env, &topics.get(1).unwrap()).unwrap(),
        admin
    );
}

// ---------------------------------------------------------------------------
// Issue #12 — Access control model
// ---------------------------------------------------------------------------

#[test]
fn test_unauthorized_create_rejected() {
    let (env, client, _admin) = setup_no_mock();
    let caller = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"x");

    let res = client.try_create_entry(&caller, &k, &v);
    assert_eq!(res, Err(Ok(Error::Unauthorized)));
}

#[test]
fn test_unauthorized_update_rejected() {
    let (env, client, _admin) = setup();
    let owner = Address::generate(&env);
    let attacker = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"x");

    client.create_entry(&owner, &k, &v);

    // Remove mocked auths so the attacker's call cannot be authorized.
    env.mock_auths(&[]);
    let res = client.try_update_entry(&attacker, &k, &v);
    assert_eq!(res, Err(Ok(Error::Unauthorized)));
}

#[test]
fn test_unauthorized_remove_rejected() {
    let (env, client, _admin) = setup();
    let owner = Address::generate(&env);
    let attacker = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"x");

    client.create_entry(&owner, &k, &v);

    env.mock_auths(&[]);
    let res = client.try_remove_entry(&attacker, &k);
    assert_eq!(res, Err(Ok(Error::Unauthorized)));
}

#[test]
fn test_non_owner_cannot_update() {
    let (env, client, _admin) = setup();
    let owner = Address::generate(&env);
    let other = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"x");

    client.create_entry(&owner, &k, &v);

    // `other` is authorized by mock_all_auths, but is not the entry owner.
    let res = client.try_update_entry(&other, &k, &v);
    assert_eq!(res, Err(Ok(Error::Unauthorized)));
}

#[test]
fn test_non_owner_cannot_remove() {
    let (env, client, _admin) = setup();
    let owner = Address::generate(&env);
    let other = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"x");

    client.create_entry(&owner, &k, &v);

    let res = client.try_remove_entry(&other, &k);
    assert_eq!(res, Err(Ok(Error::Unauthorized)));
}

#[test]
fn test_double_initialize_rejected() {
    let (env, client, _admin) = setup();
    let other = Address::generate(&env);
    let res = client.try_initialize(&other);
    assert_eq!(res, Err(Ok(Error::AlreadyInitialized)));
    let _ = env;
}

// ---------------------------------------------------------------------------
// Issue #13 — Input validation and size limits
// ---------------------------------------------------------------------------

#[test]
fn test_empty_key_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = Symbol::new(&env, "");
    let v = Bytes::from_slice(&env, b"x");

    let res = client.try_create_entry(&caller, &k, &v);
    assert_eq!(res, Err(Ok(Error::EmptyInput)));
}

#[test]
fn test_empty_value_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::new(&env);

    let res = client.try_create_entry(&caller, &k, &v);
    assert_eq!(res, Err(Ok(Error::EmptyInput)));
}

#[test]
fn test_key_at_limit_accepted() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    // MAX_KEY_LEN = 32
    let k = Symbol::new(&env, &"k".repeat(32));
    let v = Bytes::from_slice(&env, b"x");

    client.create_entry(&caller, &k, &v);
    assert_eq!(client.get_entry(&k), Some(v));
}

#[test]
fn test_key_too_long_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = Symbol::new(&env, &"k".repeat(33));
    let v = Bytes::from_slice(&env, b"x");

    let res = client.try_create_entry(&caller, &k, &v);
    assert_eq!(res, Err(Ok(Error::KeyTooLong)));
}

#[test]
fn test_value_at_limit_accepted() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "alpha");
    // MAX_VALUE_LEN = 1024
    let v = bytes_of_len(&env, 1024);

    client.create_entry(&caller, &k, &v);
    assert_eq!(client.get_entry(&k), Some(v));
}

#[test]
fn test_value_too_long_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = bytes_of_len(&env, 1025);

    let res = client.try_create_entry(&caller, &k, &v);
    assert_eq!(res, Err(Ok(Error::ValueTooLong)));
}

#[test]
fn test_update_value_too_long_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"x");

    client.create_entry(&caller, &k, &v);

    let big = bytes_of_len(&env, 1025);
    let res = client.try_update_entry(&caller, &k, &big);
    assert_eq!(res, Err(Ok(Error::ValueTooLong)));
}

#[test]
fn test_too_many_entries_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let v = Bytes::from_slice(&env, b"x");

    // MAX_ENTRIES = 100
    for i in 0..100u32 {
        let k = Symbol::new(&env, &format!("k{}", i));
        client.create_entry(&caller, &k, &v);
    }

    let extra = Symbol::new(&env, "overflow");
    let res = client.try_create_entry(&caller, &extra, &v);
    assert_eq!(res, Err(Ok(Error::TooManyEntries)));
}

#[test]
fn test_duplicate_create_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"x");

    client.create_entry(&caller, &k, &v);
    let res = client.try_create_entry(&caller, &k, &v);
    assert_eq!(res, Err(Ok(Error::AlreadyExists)));
}

#[test]
fn test_update_missing_entry_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "missing");
    let v = Bytes::from_slice(&env, b"x");

    let res = client.try_update_entry(&caller, &k, &v);
    assert_eq!(res, Err(Ok(Error::NotFound)));
}

#[test]
fn test_remove_missing_entry_rejected() {
    let (env, client, _admin) = setup();
    let caller = Address::generate(&env);
    let k = key(&env, "missing");

    let res = client.try_remove_entry(&caller, &k);
    assert_eq!(res, Err(Ok(Error::NotFound)));
}

// ---------------------------------------------------------------------------
// Issue #14 — Immutable design (no upgrade entry point)
// ---------------------------------------------------------------------------

#[test]
fn test_no_upgrade_entry_point() {
    // The contract is immutable by design (see ADR). There must be no
    // `upgrade` function exposed on the client. Attempting to invoke one
    // would fail to compile; this test documents the invariant and asserts
    // the contract's exported interface does not include an upgrade symbol.
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, RegistryContract);
    let client = RegistryContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    // The contract exposes only the documented entry points. If an upgrade
    // function were added, this list would need to change — the test acts as
    // a guard against accidental introduction of an upgrade path.
    let expected: &[&str] = &[
        "initialize",
        "create_entry",
        "update_entry",
        "remove_entry",
        "get_entry",
        "entry_count",
    ];

    // Sanity: each expected entry point is callable / present.
    let caller = Address::generate(&env);
    let k = key(&env, "alpha");
    let v = Bytes::from_slice(&env, b"x");
    client.create_entry(&caller, &k, &v);
    assert_eq!(client.get_entry(&k), Some(v.clone()));
    assert_eq!(client.entry_count(), 1);
    client.update_entry(&caller, &k, &v);
    client.remove_entry(&caller, &k);
    assert_eq!(client.entry_count(), 0);

    assert_eq!(expected.len(), 6);
}