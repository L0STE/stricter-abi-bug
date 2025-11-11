use pinocchio::{
    ProgramResult, 
    account_info::AccountInfo, 
    program_error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;
use crate::token2022::{InitializeGroup, InitializeGroupPointer, InitializeMint2, constants::TOKEN_2022_PROGRAM_ID};

use crate::token2022::constants::{TOKEN_2022_MINT_LEN, TOKEN_2022_MINT_BASE_LEN, TOKEN_2022_GROUP_POINTER_LEN, TOKEN_2022_GROUP_LEN};

pub fn process_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [mint, payer, _token_2022_program, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Create mint account
    let space = TOKEN_2022_MINT_LEN
        + TOKEN_2022_MINT_BASE_LEN
        + TOKEN_2022_GROUP_POINTER_LEN;

    let lamports = Rent::get()?.minimum_balance(
        space
            + TOKEN_2022_GROUP_LEN
    );

    CreateAccount {
        from: payer,
        to: mint,
        lamports,
        space: space as u64,
        owner: &TOKEN_2022_PROGRAM_ID,
    }
    .invoke()?;

    // Initialize Group Pointer
    InitializeGroupPointer {
        mint: mint,
        authority: mint.key(),
        group_address: mint.key(),
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
    InitializeGroup {
        group: mint,
        mint: mint,
        mint_authority: mint,
        update_authority: mint.key(),
        max_size: 100,
    }
    .invoke()?;

    Ok(())
}