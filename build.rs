use ethers::{prelude::Abigen, solc::Solc};

const UTXO_INTERFACE_PATH: &'static str = "./contracts/contracts/interfaces/IUTXO.sol";
const UTXO_INTERFACE_NAME: &'static str = "IUTXO";

fn main() -> eyre::Result<()> {
    let out_file = format!("{}/{}.rs", std::env::var("OUT_DIR")?, UTXO_INTERFACE_NAME);

    let contract_name = UTXO_INTERFACE_NAME;
    let contract = UTXO_INTERFACE_PATH;

    let contracts = Solc::default().compile_source(&contract)?;
    let abi = contracts
        .get(&contract, &contract_name)
        .unwrap()
        .abi
        .unwrap();

    let abi = serde_json::to_string(abi)?;

    Abigen::new(&contract_name, abi)?
        .generate()?
        .write_to_file(out_file)?;

    Ok(())
}
