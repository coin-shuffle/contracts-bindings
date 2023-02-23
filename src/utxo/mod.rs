pub mod errors;
pub mod iutxo;
pub mod types;

use ethers_core::abi::Hash;
use ethers_core::types::Address;
use ethers_core::types::{H256, U256};
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::LocalWallet;
use std::str::FromStr;
use std::sync::Arc;

use self::errors::Error;
use self::types::{Input, Output, Utxo};

#[async_trait::async_trait(?Send)]
pub trait Contract {
    type Error: std::error::Error;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error>;

    async fn list_utxos_by_address(
        &self,
        address: Address,
        offset: u128,
        limit: u128,
    ) -> Result<Vec<Utxo>, Self::Error>;

    async fn transfer(&self, inputs: Vec<Input>, outputs: Vec<Output>)
        -> Result<H256, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct Connector<M: Middleware> {
    utxo_contract: iutxo::IUTXO<M>,
}

impl Connector<Provider<Http>> {
    pub fn new(rpc_url: String, address: String) -> Result<Self, Error<Provider<Http>>> {
        let utxo_contract = iutxo::IUTXO::new(
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
        let utxo_contract = iutxo::IUTXO::new(
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

#[async_trait::async_trait(?Send)]
impl Contract for Connector<Provider<Http>> {
    type Error = Error<Provider<Http>>;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error> {
        let err = match self.utxo_contract.get_utxo_by_id(utxo_id).call().await {
            Ok(utxo) => return Ok(Some(utxo.into())),
            Err(err) => err,
        };

        if format!("{}", err).contains(UTXO_NOT_FOUND) {
            return Ok(None);
        }

        Err(Error::GetUTXOById(err))
    }

    async fn list_utxos_by_address(
        &self,
        address: Address,
        offset: u128,
        limit: u128,
    ) -> Result<Vec<Utxo>, Self::Error> {
        let utxos = self
            .utxo_contract
            .list_utx_os_by_address(address, U256::from(offset), U256::from(limit))
            .call()
            .await
            .map_err(|err| Error::ListUTXO(err))?;

        Ok(utxos
            .into_iter()
            .map(|utxo| utxo.into())
            .collect::<Vec<Utxo>>())
    }

    /// # Panics
    ///
    /// Private key is not set for this method. Use Connector::with_priv_key()
    async fn transfer(&self, _: Vec<Input>, _: Vec<Output>) -> Result<H256, Self::Error> {
        unimplemented!()
    }
}

#[async_trait::async_trait(?Send)]
impl Contract for Connector<SignerMiddleware<Provider<Http>, LocalWallet>> {
    type Error = Error<SignerMiddleware<Provider<Http>, LocalWallet>>;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error> {
        let err = match self.utxo_contract.get_utxo_by_id(utxo_id).call().await {
            Ok(utxo) => return Ok(Some(utxo.into())),
            Err(err) => err,
        };

        if format!("{}", err).contains(UTXO_NOT_FOUND) {
            return Ok(None);
        }

        Err(Error::GetUTXOById(err))
    }

    async fn list_utxos_by_address(
        &self,
        address: Address,
        offset: u128,
        limit: u128,
    ) -> Result<Vec<Utxo>, Self::Error> {
        let utxos = self
            .utxo_contract
            .list_utx_os_by_address(address, U256::from(offset), U256::from(limit))
            .call()
            .await
            .map_err(|err| Error::ListUTXO(err))?;

        Ok(utxos
            .into_iter()
            .map(|utxo| utxo.into())
            .collect::<Vec<Utxo>>())
    }

    async fn transfer(
        &self,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
    ) -> Result<Hash, Self::Error> {
        let inputs = inputs.into_iter().map(|i| i.into()).collect();
        let outputs = outputs.into_iter().map(|o| o.into()).collect();

        Ok(*self
            .utxo_contract
            .transfer(inputs, outputs)
            .send()
            .await
            .map_err(Error::Transfer)?)
    }
}
