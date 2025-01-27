use serde::de::DeserializeOwned;

use bank::query::QueryMessage;
use bank::{Bank, BankConfig, TokenConfig};
use sov_modules_api::mocks::MockContext;
use sov_modules_api::{Module, PublicKey, Spec};
use sov_state::mocks::MockStorageSpec;
use sov_state::{ProverStorage, WorkingSet};

pub type C = MockContext;
pub type Storage = ProverStorage<MockStorageSpec>;

pub fn query_and_deserialize<R: DeserializeOwned>(
    bank: &Bank<C>,
    query: QueryMessage<C>,
    working_set: &mut WorkingSet<Storage>,
) -> R {
    let response = bank.query(query, working_set);
    serde_json::from_slice(&response.response).expect("Failed to deserialize response json")
}

pub fn generate_address(key: &str) -> <C as Spec>::Address {
    let pk = <C as Spec>::PublicKey::try_from(key).unwrap();
    pk.to_address::<<C as Spec>::Address>()
}

pub fn create_bank_config_with_token(
    addresses_count: usize,
    initial_balance: u64,
) -> BankConfig<C> {
    let address_and_balances = (0..addresses_count)
        .map(|i| {
            let key = format!("key_{}", i);
            let addr = generate_address(&key);
            (addr, initial_balance)
        })
        .collect();

    let token_config = TokenConfig {
        token_name: "InitialToken".to_owned(),
        address_and_balances,
    };

    BankConfig {
        tokens: vec![token_config],
    }
}
