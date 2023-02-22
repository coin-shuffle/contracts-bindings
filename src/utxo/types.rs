use ethers_core::{
    types::Bytes,
    types::{Address, U256},
};

use super::iutxo;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Output {
    pub amount: U256,
    pub owner: Address,
}

impl From<iutxo::Output> for Output {
    fn from(output: iutxo::Output) -> Self {
        Self {
            amount: output.0,
            owner: output.1,
        }
    }
}

impl Into<iutxo::Output> for Output {
    fn into(self) -> iutxo::Output {
        iutxo::Output(self.amount, self.owner)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub id: U256,
    pub signature: Bytes,
}

impl From<iutxo::Input> for Input {
    fn from(input: iutxo::Input) -> Self {
        Self {
            id: input.0,
            signature: input.1,
        }
    }
}

impl Into<iutxo::Input> for Input {
    fn into(self) -> iutxo::Input {
        iutxo::Input(self.id, self.signature)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Utxo {
    pub id: U256,
    pub token: Address,
    pub amount: U256,
    pub owner: Address,
    pub is_spent: bool,
}

impl From<iutxo::Utxo> for Utxo {
    fn from(utxo: iutxo::Utxo) -> Self {
        Self {
            id: utxo.0,
            token: utxo.1,
            amount: utxo.2,
            owner: utxo.3,
            is_spent: utxo.4,
        }
    }
}

impl Into<iutxo::Utxo> for Utxo {
    fn into(self) -> iutxo::Utxo {
        iutxo::Utxo(self.id, self.token, self.amount, self.owner, self.is_spent)
    }
}
