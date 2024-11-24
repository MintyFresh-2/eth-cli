use super::consts::{AMOY_ADDR, AMOY_PRIV_KEY, RPC_URL};
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::*,
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

pub async fn my_send_transaction() -> Result<()> {
    let signer = PrivateKeySigner::from_slice(
        alloy::primitives::hex::decode(AMOY_PRIV_KEY)
            .unwrap()
            .as_slice(),
    )?;

    let wallet = EthereumWallet::new(signer);

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_http(RPC_URL.parse()?);

    let my_address = Address::new(alloy::primitives::hex::decode_to_array(AMOY_ADDR).unwrap());
    // let my_account = provider.get_account(my_address).latest();

    let tx = TransactionRequest::default()
        .with_to(my_address)
        .with_nonce(0)
        .with_chain_id(provider.get_chain_id().await?)
        .with_value(U256::from(100))
        .with_gas_limit(21_000)
        .with_max_priority_fee_per_gas(100_000_000_000)
        .with_max_fee_per_gas(200_000_000_000);

    let tx_envelope = tx.build(&provider.wallet()).await?;

    let receipt = provider
        .send_tx_envelope(tx_envelope)
        .await?
        .get_receipt()
        .await?;

    println!("Sent transaction: {}", receipt.transaction_hash);

    Ok(())
}
