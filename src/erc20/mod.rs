pub mod errors;
pub mod ierc20;

use async_trait::async_trait;
use ethers_core::types::{Address, U256};

use self::errors::Error;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Contract {
    async fn approve(&self, spender: Address, value: U256) -> Result<bool, Error>;

    fn approve_calldata(&self, spender: Address, value: U256) -> Result<Option<Vec<u8>>, Error>;
}

#[derive(Clone)]
pub struct Connector {
    inner: ierc20::IERC20,
}
