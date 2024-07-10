use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    message::Message,
    native_token::sol_to_lamports,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    system_program,
};
use std::error::Error;
use std::str::FromStr;
use solana_sdk::instruction::AccountMeta;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the RPC client
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::finalized());

    // Replace with your own keypair path or use solana-keygen to generate a new one
    // let from_keypair_path = "/path/to/your/keypair.json";
    let keypair_bytes: [u8; 64] = [
        65, 69, 152, 28, 242, 81, 119, 57, 233, 182, 41, 50, 69, 188, 83, 57, 97, 190, 64, 143,
        229, 40, 106, 12, 23, 165, 120, 2, 241, 209, 116, 188, 43, 25, 115, 68, 159, 192, 198, 208,
        117, 98, 48, 205, 62, 143, 129, 111, 96, 39, 47, 173, 136, 141, 58, 249, 164, 85, 189, 39,
        0, 159, 5, 18,
    ];
    let from_keypair = Keypair::from_bytes(keypair_bytes.as_slice())?;

    // Replace with the recipient's wallet address
    // let to_pubkey_str = "RecipientPublicKeyHere";
    let to_pubkey = Pubkey::from_str("8sWNQqKJGA9kwgXfLf9AcVe3ghmCoUTqncjEuKPWnTJs")?;

    // Amount to send in SOL
    let amount_in_sol = 0.01;
    let amount_in_lamports = sol_to_lamports(amount_in_sol);

    let sender_account_balance = lamports_to_sol(client.get_balance(&from_keypair.pubkey())?);
    let receiver_account_balance = lamports_to_sol(client.get_balance(&to_pubkey)?);

    println!(
        "sender balance before transaction: {} SOL",
        sender_account_balance
    );
    println!(
        "receiver balance before transaction: {} SOL",
        receiver_account_balance
    );

    let program_account_pubkey = Pubkey::from_str("G6aWSRtRo36DAi2Qd5jfPQEyfFv8Z7thcmvdZCzoXkZy")?;

    let transfer_instruction = Instruction::new_with_bincode(
        program_account_pubkey,
        &amount_in_lamports.to_le_bytes(),
        vec![
            AccountMeta::new(from_keypair.pubkey(), true),
            AccountMeta::new(to_pubkey, false),
            AccountMeta::new(system_program::id(), false)
        ],
    );

    // Create a transfer instruction
    // let transfer_instruction =
    //     system_instruction::transfer(&from_keypair.pubkey(), &to_pubkey, amount_in_lamports);

    // Create a message
    let message = Message::new(&[transfer_instruction], Some(&from_keypair.pubkey()));

    // Create a transaction
    let mut transaction = Transaction::new_unsigned(message);

    // Get the recent blockhash
    let latest_blockhash = client.get_latest_blockhash()?;

    // Sign the transaction
    transaction.try_sign(&[&from_keypair], latest_blockhash)?;

    // Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("Transaction successful with signature: {}", signature);

    let sender_account_balance = lamports_to_sol(client.get_balance(&from_keypair.pubkey())?);
    let receiver_account_balance = lamports_to_sol(client.get_balance(&to_pubkey)?);

    println!(
        "sender balance after transaction: {} SOL",
        sender_account_balance
    );
    println!(
        "receiver balance after transaction: {} SOL",
        receiver_account_balance
    );

    Ok(())
}
