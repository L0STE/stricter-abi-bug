use pinocchio::{
    ProgramResult, account_info::AccountInfo, log::sol_log, program_entrypoint, program_error::ProgramError, pubkey::Pubkey, sysvars::{Sysvar, rent::Rent}
};
use pinocchio_system::instructions::CreateAccount;

use crate::token2022::{InitializeMetadata, InitializeMetadataPointer, InitializeMint2, constants::{ TOKEN_2022_METADATA_LEN, TOKEN_2022_METADATA_POINTER_LEN, TOKEN_2022_MINT_BASE_LEN, TOKEN_2022_MINT_LEN, TOKEN_2022_PROGRAM_ID}};

#[cfg(test)]
pub mod tests;
pub mod token2022;
pub mod utils;
pub mod instructions;

program_entrypoint!(process_instruction);

// srsUi2TVUUCyGcZdopxJauk8ZBzgAaHHZCVUhm5ifPa
pub const ID: Pubkey = [
    0x0d, 0x07, 0x6d, 0xd2, 0x25, 0x68, 0x1a, 0x37, 
    0x2b, 0x70, 0x18, 0x49, 0xae, 0xc6, 0x09, 0x13,
    0x88, 0xf0, 0x8d, 0x04, 0x7c, 0x42, 0x8c, 0xcd, 
    0x0d, 0xda, 0x8a, 0x49, 0x4a, 0xcb, 0x24, 0x1d,
];

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, _) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match discriminator {
        0x01 => instructions::reproduction1::process_instruction(accounts),
        0x02 => instructions::reproduction2::process_instruction(accounts),
        0x03 => instructions::reproduction3::process_instruction(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
