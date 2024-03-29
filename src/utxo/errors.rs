use ethers_contract::ContractError;
use ethers_middleware::signer::SignerMiddlewareError;
use ethers_providers::Middleware;
use ethers_signers::{Signer, WalletError};
use rustc_hex::FromHexError;

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

    #[error("failed to do list utxos: {0}")]
    ListUTXO(ContractError<M>),

    #[error("failed to do transfer: {0}")]
    Transfer(ContractError<M>),
}

#[derive(thiserror::Error, Debug)]
pub enum ConnectorWithSignerError<M: Middleware, S: Signer> {
    #[error("failed to create connector with signer")]
    Signer(#[from] SignerMiddlewareError<M, S>),
    #[error("failed to parse address: {0}")]
    ParseAddress(#[from] FromHexError),
    #[error("failed to parse service url: {0}")]
    ParseServiceURL(#[from] url::ParseError),
    #[error("failed to parse private key: {0}")]
    ParsePrivateKey(#[from] WalletError),
}
