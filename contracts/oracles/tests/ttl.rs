#![cfg(test)]
extern crate std;

use oracles::{CapabilityFlag, OraclesContract, OraclesContractClient};
use soroban_sdk::{contract, contractimpl, testutils::Address as _, Address, Env, String};

#[contract]
pub struct OracleWithoutPrice;

#[contractimpl]
impl OracleWithoutPrice {
    pub fn capabilities(_env: Env) -> u64 {
        CapabilityFlag::GET_PRICE
    }
}

#[test]
fn test_ttl_bump_hot_read_paths() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(OraclesContract, ());
    let client = OraclesContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    let oracle = env.register(OracleWithoutPrice, ());
    client.add_oracle(&admin, &oracle).unwrap();

    // list_oracles should succeed and bump TTL
    let oracles = client.list_oracles();
    assert_eq!(oracles.len(), 1);
    assert_eq!(oracles.get(0).unwrap(), oracle);

    // This contract advertises GET_PRICE but omits the price entrypoints, so
    // the calls fail after exercising the registry TTL bump.
    let feed_id = String::from_str(&env, "BTC/USD");
    
    let res1 = client.try_get_price(&oracle, &feed_id);
    assert!(res1.is_err(), "Expected error because get_price is absent");

    let res2 = client.try_get_price_data(&oracle, &feed_id);
    assert!(res2.is_err(), "Expected error because price entrypoints are absent");

    // For is_oracle_healthy, it intercepts the error and returns false if the contract invocation fails
    let is_healthy = client.is_oracle_healthy(&oracle, &feed_id);
    assert_eq!(is_healthy, false);
}
