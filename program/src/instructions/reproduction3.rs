use pinocchio::{
    ProgramResult, 
    account_info::AccountInfo, 
    program_error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::CreateAccount;
use crate::token2022::{InitializeGroup, InitializeGroupMemberPointer, InitializeGroupPointer, InitializeMember, InitializeMint2, constants::{TOKEN_2022_MEMBER_LEN, TOKEN_2022_MEMBER_POINTER_LEN, TOKEN_2022_PROGRAM_ID}};

use crate::token2022::constants::{TOKEN_2022_MINT_LEN, TOKEN_2022_MINT_BASE_LEN, TOKEN_2022_GROUP_POINTER_LEN, TOKEN_2022_GROUP_LEN};

pub fn process_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [mint, group, payer, _token_2022_program, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Create Group account
    let space = TOKEN_2022_MINT_LEN
        + TOKEN_2022_MINT_BASE_LEN
        + TOKEN_2022_GROUP_POINTER_LEN;

    let lamports = Rent::get()?.minimum_balance(
        space
            + TOKEN_2022_GROUP_LEN
    );

    CreateAccount {
        from: payer,
        to: group,
        lamports,
        space: space as u64,
        owner: &TOKEN_2022_PROGRAM_ID,
    }
    .invoke()?;

    // Initialize Group Pointer
    InitializeGroupPointer {
        mint: group,
        authority: group.key(),
        group_address: group.key(),
    }
    .invoke()?;

    // Initialize Mint
    InitializeMint2 {
        mint: group,
        decimals: 0,
        mint_authority: group.key(),
        freeze_authority: Some(group.key()),
    }
    .invoke()?;

    // Initialize Metadata
    InitializeGroup {
        group: group,
        mint: group,
        mint_authority: group,
        update_authority: group.key(),
        max_size: 100,
    }
    .invoke()?;


    // Create Member account
    let space = TOKEN_2022_MINT_LEN
        + TOKEN_2022_MINT_BASE_LEN
        + TOKEN_2022_MEMBER_POINTER_LEN;

    let lamports = Rent::get()?.minimum_balance(
        space
            + TOKEN_2022_MEMBER_LEN
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
    InitializeGroupMemberPointer {
        mint: mint,
        authority: mint.key(),
        member_address: mint.key(),
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
    InitializeMember {
        member: mint,
        mint: mint,
        mint_authority: mint,
        group: group,
        group_update_authority: group,
    }
    .invoke()?;

    Ok(())
}