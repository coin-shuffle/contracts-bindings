use ethers_contract::Abigen;
use ethers_solc::Solc;

const UTXO_INTERFACE_CONTRACT: &str = "IUTXO";
const INTERFACES_PATH: &str = "./contracts/contracts/interfaces";

fn main() -> eyre::Result<()> {
    generate_contracts(
        &[
            UTXO_INTERFACE_CONTRACT, // Add more contracts here
        ],
        INTERFACES_PATH,
    )?;

    Ok(())
}

fn generate_contracts(contracts_names: &[&str], path: &str) -> eyre::Result<()> {
    let out_dir = std::env::var("OUT_DIR")?;

    let contracts = Solc::default().compile_source(path)?;

    if contracts.has_error() {
        return Err(eyre::eyre!(
            "Failed to compile contracts: {:?}",
            contracts.errors
        ));
    }

    for contract_name in contracts_names {
        let contract_path = format!("{}/{}.sol", path, contract_name);

        let abi = contracts
            .get(&contract_path, contract_name)
            .ok_or(eyre::eyre!(
                "Contract not found: name={} path={}",
                contract_name,
                contract_path
            ))?
            .abi
            .ok_or(eyre::eyre!(
                "Contract abi not found: name={} path={}",
                contract_name,
                contract_path
            ))?;

        let abi = serde_json::to_string(abi)?;

        let out_file = format!("{}/{}.rs", out_dir, contract_name);

        Abigen::new(contract_name.to_owned(), abi)?
            .generate()?
            .write_to_file(out_file)?;
    }

    Ok(())
}
