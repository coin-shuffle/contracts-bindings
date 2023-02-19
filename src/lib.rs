macro_rules! include_contract {
    ($contract:tt) => {
        include!(concat!(env!("OUT_DIR"), "/", $contract, ".rs"));
    };
}

pub mod utxo {
    include_contract!("IUTXO");
}
