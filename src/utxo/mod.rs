pub mod errors;
pub mod iutxo;
pub mod types;

use async_trait::async_trait;
use ethers_contract::ContractError;
use ethers_contract::EthError;
use ethers_core::abi::Hash;
use ethers_core::types::Address;
use ethers_core::types::Selector;
use ethers_core::types::{H256, U256};
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::LocalWallet;
use std::str::FromStr;
use std::sync::Arc;

use self::errors::ConnectorWithSignerError;
use self::errors::Error;
use self::types::{Input, Output, Utxo};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Contract {
    type Error: std::error::Error;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error>;

    async fn list_utxos_by_address(
        &self,
        address: Address,
        offset: U256,
        limit: U256,
    ) -> Result<Vec<Utxo>, Self::Error>;

    async fn utxo_length(&self) -> Result<U256, Self::Error>;

    async fn transfer(&self, inputs: Vec<Input>, outputs: Vec<Output>)
        -> Result<H256, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct Connector<M: Middleware> {
    utxo_contract: iutxo::IUTXO<M>,
}

impl<M> Connector<M>
where
    M: Middleware,
{
    async fn utxos_by_address(
        &self,
        address: Address,
        offset: U256,
        limit: U256,
    ) -> Result<Vec<Utxo>, Error<M>> {
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

    async fn utxos_by_id(&self, id: U256) -> Result<Option<Utxo>, Error<M>> {
        let err = match self.utxo_contract.get_utxo_by_id(id).call().await {
            Ok(utxo) => return Ok(Some(utxo.into())),
            Err(err) => err,
        };

        if Self::is_revert_reason(&err, iutxo::UtxoNotFound::selector()) {
            Ok(None)
        } else {
            Err(Error::GetUTXOById(err))
        }
    }

    /// compare selector with the first 4 bytes of the revert reason
    fn is_revert_reason(err: &ContractError<M>, selector: Selector) -> bool {
        if let ContractError::Revert(bytes) = err {
            for i in 0..selector.len() {
                if bytes[i] != selector[i] {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    async fn utxo_length(&self) -> Result<U256, Error<M>> {
        self.utxo_contract
            .get_utx_os_length()
            .call()
            .await
            .map_err(Error::UtxoLength)
    }
}

impl Connector<Provider<Http>> {
    pub fn from_raw(rpc_url: String, address: String) -> Result<Self, Error<Provider<Http>>> {
        let address = Address::from_str(&address)?;
        let rpc_url = rpc_url.parse()?;

        Ok(Self::new(rpc_url, address))
    }

    pub fn new(rpc_url: url::Url, address: Address) -> Self {
        let provider = Provider::new(Http::new(rpc_url));
        let utxo_contract = iutxo::IUTXO::new(address, Arc::new(provider));

        Self { utxo_contract }
    }

    pub fn with_provider(address: Address, provider: Arc<Provider<Http>>) -> Self {
        let utxo_contract = iutxo::IUTXO::new(address, provider);

        Self { utxo_contract }
    }
}

impl Connector<SignerMiddleware<Provider<Http>, LocalWallet>> {
    pub async fn with_priv_key(
        rpc_url: String,
        address: String,
        priv_key: String,
    ) -> Result<Self, ConnectorWithSignerError<Provider<Http>, LocalWallet>> {
        let utxo_contract = iutxo::IUTXO::new(
            Address::from_str(address.as_str())?,
            Arc::new(
                SignerMiddleware::new_with_provider_chain(
                    Provider::<Http>::try_from(rpc_url.as_str())?,
                    LocalWallet::from_str(priv_key.as_str())?,
                )
                .await?,
            ),
        );

        Ok(Self { utxo_contract })
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Contract for Connector<Provider<Http>> {
    type Error = Error<Provider<Http>>;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error> {
        self.utxos_by_id(utxo_id).await
    }

    async fn list_utxos_by_address(
        &self,
        address: Address,
        offset: U256,
        limit: U256,
    ) -> Result<Vec<Utxo>, Self::Error> {
        self.utxos_by_address(address, offset, limit).await
    }

    async fn utxo_length(&self) -> Result<U256, Self::Error> {
        self.utxo_length().await
    }

    /// # Panics
    ///
    /// Private key is not set for this method. Use Connector::with_priv_key()
    async fn transfer(&self, _: Vec<Input>, _: Vec<Output>) -> Result<H256, Self::Error> {
        unimplemented!()
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Contract for Connector<SignerMiddleware<Provider<Http>, LocalWallet>> {
    type Error = Error<SignerMiddleware<Provider<Http>, LocalWallet>>;

    async fn get_utxo_by_id(&self, utxo_id: U256) -> Result<Option<Utxo>, Self::Error> {
        self.utxos_by_id(utxo_id).await
    }

    async fn utxo_length(&self) -> Result<U256, Self::Error> {
        self.utxo_length().await
    }

    async fn list_utxos_by_address(
        &self,
        address: Address,
        offset: U256,
        limit: U256,
    ) -> Result<Vec<Utxo>, Self::Error> {
        self.utxos_by_address(address, offset, limit).await
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
