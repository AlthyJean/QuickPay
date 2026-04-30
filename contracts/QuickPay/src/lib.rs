#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol};

#[contracttype]
pub struct Escrow {
    pub client: Address,
    pub freelancer: Address,
    pub amount: i128,
    pub released: bool,
}

#[contract]
pub struct QuickPayContract;

#[contractimpl]
impl QuickPayContract {

    // Create escrow
    pub fn create_escrow(env: Env, client: Address, freelancer: Address, amount: i128) {
        let escrow = Escrow {
            client: client.clone(),
            freelancer,
            amount,
            released: false,
        };

        env.storage().instance().set(&Symbol::short("ESCROW"), &escrow);
    }

    // Release payment
    pub fn release_payment(env: Env, client: Address) {
        let mut escrow: Escrow = env.storage().instance().get(&Symbol::short("ESCROW")).unwrap();

        // Only client can release
        client.require_auth();

        if escrow.released {
            panic!("Already released");
        }

        escrow.released = true;
        env.storage().instance().set(&Symbol::short("ESCROW"), &escrow);
    }

    // View escrow
    pub fn get_escrow(env: Env) -> Escrow {
        env.storage().instance().get(&Symbol::short("ESCROW")).unwrap()
    }
}