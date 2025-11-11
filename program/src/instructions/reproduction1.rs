use pinocchio::{
    ProgramResult, 
    account_info::AccountInfo, 
    program_error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;
use crate::token2022::{InitializeMetadata, InitializeMetadataPointer, InitializeMint2, constants::TOKEN_2022_PROGRAM_ID};

use crate::token2022::constants::{TOKEN_2022_MINT_LEN, TOKEN_2022_MINT_BASE_LEN, TOKEN_2022_METADATA_POINTER_LEN, TOKEN_2022_METADATA_LEN};

pub fn process_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [mint, payer, _token_2022_program, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Fake Metadata that has
    // - name: "test"
    // - symbol: "SRS"
    // - uri: "test"
    let metadata = &[4, 0, 0, 0, 116, 101, 115, 116, 3, 0, 0, 0, 83, 82, 83, 4, 0, 0, 0, 116, 101, 115, 116];

    // Create mint account
    let space = TOKEN_2022_MINT_LEN
        + TOKEN_2022_MINT_BASE_LEN
        + TOKEN_2022_METADATA_POINTER_LEN;

    let lamports = Rent::get()?.minimum_balance(
        space
            + metadata.len()
            + TOKEN_2022_METADATA_LEN
    );

    CreateAccount {
        from: payer,
        to: mint,
        lamports,
        space: space as u64,
        owner: &TOKEN_2022_PROGRAM_ID,
    }
    .invoke()?;

    // Initialize Metadata Pointer
    InitializeMetadataPointer {
        mint: mint,
        authority: mint.key(),
        metadata_address: mint.key(),
    }
    .invoke()?;

    // Initialize Mint
    InitializeMint2 {
        mint: mint,
        decimals: 0,
        mint_authority: mint.key(),
        freeze_authority: Some(mint.key()),
    }
    .invoke()?;

    // Initialize Metadata
    InitializeMetadata {
        metadata: mint,
        mint: mint,
        update_authority: mint,
        mint_authority: mint,
        metadata_data: metadata,
    }
    .invoke()?;

    Ok(())
}