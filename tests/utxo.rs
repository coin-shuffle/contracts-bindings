use coin_shuffle_contracts_bindings::utxo::{self, Contract};
use ethers::types::U256;

const ENV_UTXO_ADDR: &str = "UTXO_ADDR";

const GANACHE_PRIVATE_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

const GANACHE_RPC_URL: &str = "http://localhost:8545";

#[tokio::test]
async fn test_get_utxo_by_id() {
    let utxo_addr = std::env::var(ENV_UTXO_ADDR).expect("UTXO_ADDR not set");

    let connector = utxo::Connector::new(GANACHE_RPC_URL.to_string(), utxo_addr).unwrap();

    let result = connector.get_utxo_by_id(U256::from(1)).await;

    println!("{}", result.err().unwrap());
}
