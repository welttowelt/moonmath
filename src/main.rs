use ethers::prelude::*;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::main;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Deine RPC-URL und der private Schlüssel
    let rpc_url = "https://sepolia-rpc.kakarot.org";
    let private_key = "ed8824752e695757b46c5a4a27ece67e406da440feb28c451f419a0718756b17";

    // Verbinde mit dem Provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Initialisiere die Wallet
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(1802203764); // Chain-ID für Kakarot

    // Verbinde die Wallet mit dem Provider
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // Contract-Adresse und ABI
    let contract_address: Address = "0x4c1fF993E16b493aEC456117d1B515567118188e".parse()?;
    let abi = include_str!("../abi/Moonmath.json"); // Stelle sicher, dass du die ABI-Datei hast

    // Initialisiere den Contract
    let contract = Contract::from_json(client, contract_address, abi.as_bytes())?;

    // Beispiel: Aufruf der performInteraction Funktion
    let tx = contract.method::<_, H256>("performInteraction", ())?
        .value(ethers::utils::parse_ether("0.00001").unwrap())
        .send()
        .await?;

    println!("Transaction hash: {:?}", tx);
    Ok(())
}

