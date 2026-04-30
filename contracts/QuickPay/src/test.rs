#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};
use crate::{QuickPayContract, QuickPayContractClient};

#[test]
fn test_happy_path() {
    let env = Env::default();
    let contract_id = env.register_contract(None, QuickPayContract);
    let client = QuickPayContractClient::new(&env, &contract_id);

    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    client.create_escrow(&user1, &user2, &1000);
    client.release_payment(&user1);

    let escrow = client.get_escrow();
    assert!(escrow.released);
}

#[test]
fn test_double_release_fail() {
    let env = Env::default();
    let contract_id = env.register_contract(None, QuickPayContract);
    let client = QuickPayContractClient::new(&env, &contract_id);

    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    client.create_escrow(&user1, &user2, &1000);
    client.release_payment(&user1);

    let result = std::panic::catch_unwind(|| {
        client.release_payment(&user1);
    });

    assert!(result.is_err());
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, QuickPayContract);
    let client = QuickPayContractClient::new(&env, &contract_id);

    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    client.create_escrow(&user1, &user2, &500);
    let escrow = client.get_escrow();

    assert_eq!(escrow.amount, 500);
}

#[test]
fn test_not_released_initially() {
    let env = Env::default();
    let contract_id = env.register_contract(None, QuickPayContract);
    let client = QuickPayContractClient::new(&env, &contract_id);

    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    client.create_escrow(&user1, &user2, &500);
    let escrow = client.get_escrow();

    assert!(!escrow.released);
}

#[test]
fn test_multiple_users() {
    let env = Env::default();
    let contract_id = env.register_contract(None, QuickPayContract);
    let client = QuickPayContractClient::new(&env, &contract_id);

    let user1 = Address::random(&env);
    let user2 = Address::random(&env);

    client.create_escrow(&user1, &user2, &2000);
    let escrow = client.get_escrow();

    assert_eq!(escrow.client, user1);
}
