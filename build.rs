use ethers::{prelude::Abigen, solc::Solc};

const UTXO_INTERFACE_PATH: &'static str = "./contracts/contracts/interfaces/IUTXO.sol";
const UTXO_INTERFACE_NAME: &'static str = "IUTXO";

fn main() -> eyre::Result<()> {
    let contracts = [(UTXO_INTERFACE_PATH, UTXO_INTERFACE_NAME)];

    for (contract_name, contract_path) in contracts {
        generate_contract(contract_name, contract_path)?;
    }

    Ok(())
}

fn generate_contract(name: &str, path: &str) -> eyre::Result<()> {
    let out_file = format!("{}/{}.rs", std::env::var("OUT_DIR")?, name);

    let contract_name = name;
    let contract = path;

    let contracts = Solc::default().compile_source(&contract)?;
    let abi = contracts
        .get(&contract, &contract_name)
        .expect(format!("failed to get contract by name: {}", contract_name).as_str())
        .abi
        .expect("failed to get contract abi");

    let abi = serde_json::to_string(abi)?;

    Abigen::new(&contract_name, abi)?
        .generate()?
        .write_to_file(out_file)?;

    Ok(())
}
