use ethers::abi::Hash;
use ethers::contract::ContractError;
use ethers::core::types::Address;
use ethers::prelude::{LocalWallet, SignerMiddleware};
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::WalletError;
use ethers::types::{H256, U256};
use rustc_hex::FromHexError;
use std::str::FromStr;
use std::sync::Arc;

use crate::macros;

macros::include_contract!("IUTXO");

#[async_trait::async_trait]
pub trait Contract {
    type Error: std::error::Error;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error>;
    async fn transfer(&self, inputs: Vec<Input>, outputs: Vec<Output>)
        -> Result<H256, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct Connector<M: Middleware> {
    utxo_contract: IUTXO<M>,
}

impl Connector<Provider<Http>> {
    pub fn new(rpc_url: String, address: String) -> Result<Self, Error<Provider<Http>>> {
        let utxo_contract = IUTXO::new(
            Address::from_str(address.as_str())?,
            Arc::new(Provider::<Http>::try_from(rpc_url.as_str())?),
        );

        Ok(Self { utxo_contract })
    }
}

impl Connector<SignerMiddleware<Provider<Http>, LocalWallet>> {
    pub fn with_priv_key(
        rpc_url: String,
        address: String,
        priv_key: String,
    ) -> Result<Self, Error<SignerMiddleware<Provider<Http>, LocalWallet>>> {
        let utxo_contract = IUTXO::new(
            Address::from_str(address.as_str())?,
            Arc::new(SignerMiddleware::new(
                Provider::<Http>::try_from(rpc_url.as_str())?,
                LocalWallet::from_str(priv_key.as_str())?,
            )),
        );

        Ok(Self { utxo_contract })
    }
}

/// Error returned by the contract if utxo is not found
///
/// TODO: Find more elegant way to handle this
const UTXO_NOT_FOUND: &str = "UTXO doesn't exist";

#[async_trait::async_trait]
impl Contract for Connector<Provider<Http>> {
    type Error = Error<Provider<Http>>;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error> {
        let err = match self.utxo_contract.get_utxo_by_id(utxo_id).call().await {
            Ok(utxo) => return Ok(Some(utxo)),
            Err(err) => err,
        };

        if format!("{}", err).contains(UTXO_NOT_FOUND) {
            return Ok(None);
        }

        Err(Error::GetUTXOById(err))
    }

    /// # Panics
    ///
    /// Private key is not set for this method. Use Connector::with_priv_key()
    async fn transfer(&self, _: Vec<Input>, _: Vec<Output>) -> Result<H256, Self::Error> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl Contract for Connector<SignerMiddleware<Provider<Http>, LocalWallet>> {
    type Error = Error<SignerMiddleware<Provider<Http>, LocalWallet>>;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error> {
        let err = match self.utxo_contract.get_utxo_by_id(utxo_id).call().await {
            Ok(utxo) => return Ok(Some(utxo)),
            Err(err) => err,
        };

        if format!("{}", err).contains(UTXO_NOT_FOUND) {
            return Ok(None);
        }

        Err(Error::GetUTXOById(err))
    }

    async fn transfer(
        &self,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
    ) -> Result<Hash, Self::Error> {
        Ok(*self
            .utxo_contract
            .transfer(inputs, outputs)
            .send()
            .await
            .map_err(Error::Transfer)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error<M: Middleware> {
    #[error("failed to parse address: {0}")]
    ParseAddress(#[from] FromHexError),

    #[error("failed to parse service url: {0}")]
    ParseServiceURL(#[from] url::ParseError),

    #[error("failed to parse private key: {0}")]
    ParsePrivateKey(#[from] WalletError),

    #[error("failed to get utxo by id: {0}")]
    GetUTXOById(ContractError<M>),

    #[error("failed to do transfer: {0}")]
    Transfer(ContractError<M>),
}
