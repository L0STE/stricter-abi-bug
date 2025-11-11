use std::str::FromStr;

use solana_account::Account;
use mollusk_svm::{result::Check, Mollusk};
use agave_feature_set::FeatureSet;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use crate::ID;

pub const PAYER: Pubkey = Pubkey::new_from_array([0xaa; 32]);
pub const MINT: Pubkey = Pubkey::new_from_array([0xbb; 32]);
pub const GROUP: Pubkey = Pubkey::new_from_array([0xcc; 32]);
pub const ACCOUNT: Pubkey = Pubkey::new_from_array([0xdd; 32]);

// TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
pub const TOKEN_2022_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x06, 0xdd, 0xf6, 0xe1, 0xee, 0x75, 0x8f, 0xde, 0x18, 0x42, 0x5d, 0xbc, 0xe4, 0x6c, 0xcd, 0xda,
    0xb6, 0x1a, 0xfc, 0x4d, 0x83, 0xb9, 0x0d, 0x27, 0xfe, 0xbd, 0xf9, 0x28, 0xd8, 0xa1, 0x8b, 0xfc,
]);

/* Helpers */

fn keyed_account_for_payer() -> (Pubkey, Account) {
    (
        PAYER,
        Account::new(100_000_000_000u64, 0, &Pubkey::default()),
    )
}

fn keyed_account_for_mint() -> (Pubkey, Account) {
    (
        MINT,
        Account::new(0, 0, &Pubkey::default()),
    )
}

fn keyed_account_for_group() -> (Pubkey, Account) {
    (
        GROUP,
        Account::new(0, 0, &Pubkey::default()),
    )
}
/* Tests */

#[test]
fn mint_token_with_stricter_abi() {
    // Payer
    let (payer, payer_data) = keyed_account_for_payer();
    // Mint
    let (mint, mint_data) = keyed_account_for_mint();
    // Token2022 Program
    let (token2022, token2022_data) = mollusk_svm_programs_token::token2022::keyed_account();
    // System Program
    let (system_program, system_program_data) = mollusk_svm::program::keyed_account_for_system_program();

    let mut mollusk = Mollusk::new(
        &Pubkey::new_from_array(ID),
        "../target/deploy/stricter_abi_bug",
    );

    let mut feature_set = FeatureSet::default();
    feature_set.activate(&Pubkey::from_str("CxeBn9PVeeXbmjbNwLv6U4C6svNxnC4JX6mfkvgeMocM").unwrap(), 0);
    mollusk.feature_set = feature_set;

    mollusk_svm_programs_token::token2022::add_program(&mut mollusk);

    let instruction = Instruction::new_with_bytes(
        Pubkey::new_from_array(ID),
        &[0x01],
        vec![
            AccountMeta::new(mint, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(token2022, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (mint, mint_data),
            (payer, payer_data),
            (token2022, token2022_data),
            (system_program, system_program_data),
        ],
        &[
            Check::success(),
        ],
    );
}

#[test]
fn mint_token() {
    // Payer
    let (payer, payer_data) = keyed_account_for_payer();
    // Mint
    let (mint, mint_data) = keyed_account_for_mint();
    // Token2022 Program
    let (token2022, token2022_data) = mollusk_svm_programs_token::token2022::keyed_account();
    // System Program
    let (system_program, system_program_data) = mollusk_svm::program::keyed_account_for_system_program();

    let mut mollusk = Mollusk::new(
        &Pubkey::new_from_array(ID),
        "../target/deploy/stricter_abi_bug",
    );

    mollusk.feature_set = FeatureSet::default();

    mollusk_svm_programs_token::token2022::add_program(&mut mollusk);

    let instruction = Instruction::new_with_bytes(
        Pubkey::new_from_array(ID),
        &[0x01],
        vec![
            AccountMeta::new(mint, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(token2022, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (mint, mint_data),
            (payer, payer_data),
            (token2022, token2022_data),
            (system_program, system_program_data),
        ],
        &[
            Check::success(),
        ],
    );
}

#[test]
fn create_group_with_stricter_abi() {
    // Payer
    let (payer, payer_data) = keyed_account_for_payer();
    // Mint
    let (mint, mint_data) = keyed_account_for_mint();
    // Token2022 Program
    let (token2022, token2022_data) = mollusk_svm_programs_token::token2022::keyed_account();
    // System Program
    let (system_program, system_program_data) = mollusk_svm::program::keyed_account_for_system_program();

    let mut mollusk = Mollusk::new(
        &Pubkey::new_from_array(ID),
        "../target/deploy/stricter_abi_bug",
    );

    let mut feature_set = FeatureSet::default();
    feature_set.activate(&Pubkey::from_str("CxeBn9PVeeXbmjbNwLv6U4C6svNxnC4JX6mfkvgeMocM").unwrap(), 0);
    mollusk.feature_set = feature_set;

    mollusk_svm_programs_token::token2022::add_program(&mut mollusk);

    let instruction = Instruction::new_with_bytes(
        Pubkey::new_from_array(ID),
        &[0x02],
        vec![
            AccountMeta::new(mint, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(token2022, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (mint, mint_data),
            (payer, payer_data),
            (token2022, token2022_data),
            (system_program, system_program_data),
        ],
        &[
            Check::success(),
        ],
    );
}

#[test]
fn create_group() {
    // Payer
    let (payer, payer_data) = keyed_account_for_payer();
    // Mint
    let (mint, mint_data) = keyed_account_for_mint();
    // Token2022 Program
    let (token2022, token2022_data) = mollusk_svm_programs_token::token2022::keyed_account();
    // System Program
    let (system_program, system_program_data) = mollusk_svm::program::keyed_account_for_system_program();

    let mut mollusk = Mollusk::new(
        &Pubkey::new_from_array(ID),
        "../target/deploy/stricter_abi_bug",
    );

    mollusk.feature_set = FeatureSet::default();

    mollusk_svm_programs_token::token2022::add_program(&mut mollusk);

    let instruction = Instruction::new_with_bytes(
        Pubkey::new_from_array(ID),
        &[0x02],
        vec![
            AccountMeta::new(mint, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(token2022, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (mint, mint_data),
            (payer, payer_data),
            (token2022, token2022_data),
            (system_program, system_program_data),
        ],
        &[
            Check::success(),
        ],
    );
}

#[test]
fn create_member_with_stricter_abi() {
    // Payer
    let (payer, payer_data) = keyed_account_for_payer();
    // Mint
    let (mint, mint_data) = keyed_account_for_mint();
    // Group
    let (group, group_data) = keyed_account_for_group();
    // Token2022 Program
    let (token2022, token2022_data) = mollusk_svm_programs_token::token2022::keyed_account();
    // System Program
    let (system_program, system_program_data) = mollusk_svm::program::keyed_account_for_system_program();

    let mut mollusk = Mollusk::new(
        &Pubkey::new_from_array(ID),
        "../target/deploy/stricter_abi_bug",
    );

    let mut feature_set = FeatureSet::default();
    feature_set.activate(&Pubkey::from_str("CxeBn9PVeeXbmjbNwLv6U4C6svNxnC4JX6mfkvgeMocM").unwrap(), 0);
    mollusk.feature_set = feature_set;

    mollusk_svm_programs_token::token2022::add_program(&mut mollusk);

    let instruction = Instruction::new_with_bytes(
        Pubkey::new_from_array(ID),
        &[0x03],
        vec![
            AccountMeta::new(mint, true),
            AccountMeta::new(group, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(token2022, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (mint, mint_data),
            (group, group_data),
            (payer, payer_data),
            (token2022, token2022_data),
            (system_program, system_program_data),
        ],
        &[
            Check::success(),
        ],
    );
}

#[test]
fn create_member() {
    // Payer
    let (payer, payer_data) = keyed_account_for_payer();
    // Mint
    let (mint, mint_data) = keyed_account_for_mint();
    // Group
    let (group, group_data) = keyed_account_for_group();
    // Token2022 Program
    let (token2022, token2022_data) = mollusk_svm_programs_token::token2022::keyed_account();
    // System Program
    let (system_program, system_program_data) = mollusk_svm::program::keyed_account_for_system_program();

    let mut mollusk = Mollusk::new(
        &Pubkey::new_from_array(ID),
        "../target/deploy/stricter_abi_bug",
    );

    mollusk.feature_set = FeatureSet::default();

    mollusk_svm_programs_token::token2022::add_program(&mut mollusk);

    let instruction = Instruction::new_with_bytes(
        Pubkey::new_from_array(ID),
        &[0x03],
        vec![
            AccountMeta::new(mint, true),
            AccountMeta::new(group, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(token2022, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (mint, mint_data),
            (group, group_data),
            (payer, payer_data),
            (token2022, token2022_data),
            (system_program, system_program_data),
        ],
        &[
            Check::success(),
        ],
    );
}