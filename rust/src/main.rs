use bitcoincore_rpc::{Auth, Client as BitcoinClient, RpcApi};
use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;

/// Call Alice's Lightning node via CLN REST API on port 3010
fn call_alice_ln(method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let rune = std::env::var("ALICE_RUNE")?;
    let url = format!("http://localhost:3010/v1/{}", method);

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&params)
        .header("Rune", rune)
        .send()?
        .json::<Value>()?;

    Ok(response)
}

/// Call Bob's Lightning node via CLN REST API on port 3011
fn call_bob_ln(method: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let rune = std::env::var("BOB_RUNE")?;
    let url = format!("http://localhost:3011/v1/{}", method);

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&params)
        .header("Rune", rune)
        .send()?
        .json::<Value>()?;

    Ok(response)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bitcoin RPC client
    let bitcoin_rpc = BitcoinClient::new(
        "http://localhost:18443",
        Auth::UserPass("alice".to_string(), "password".to_string()),
    )?;

    println!("Blockchain Info: {:?}", bitcoin_rpc.get_blockchain_info()?);
    // Get Alice's node info
    let alice_info = call_alice_ln("getinfo", Value::Null)?;
    println!("Alice Node Info: {:?}", alice_info);

    // Get Bob's node info
    let bob_info = call_bob_ln("getinfo", Value::Null)?;
    println!("Bob Node Info: {:?}", bob_info);

    // Get Alice's node ID

    // Get Bob's node ID

    // Connect Alice to Bob as a peer

    // Verify peer connection from both Alice's and Bob's perspectives

    // Create or load a mining wallet

    // Generate a new mining address from the mining wallet

    // How many blocks need to be mined to the mining address? Why?

    // Verify wallet balance

    // Create an on-chain address for Alice and send 1 BTC from mining wallet to this address

    // Mine blocks to confirm the funding transaction. How many blocks and why?

    // Open a payment channel from Alice to Bob with 500,000 satoshis capacity

    // Mine some blocks to confirm the channel opening transaction.

    // Wait a few seconds for nodes to recognize the confirmed channel

    // Verify channel is active on both Alice's side and Bob's side

    // Get channel details from both Alice's and Bob's perspectives

    // Check if Alice and Bob are peers

    // Write to out.txt

    Ok(())
}
