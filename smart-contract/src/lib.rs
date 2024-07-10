use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let sender = next_account_info(accounts_iter)?;
    let receiver = next_account_info(accounts_iter)?;

    let amount = instruction_data
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    let transfer_instruction = system_instruction::transfer(
        sender.key,
        receiver.key,
        amount
    );

    invoke(
        &transfer_instruction,
        &[sender.clone(), receiver.clone()],
    )?;


    Ok(())
}