pub mod instruction;
pub mod state;

use borsh::BorshSerialize;
use crate::instruction::ReviewInstruction;
use crate::state::AccountState;
use crate::state::ReviewError;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh0_10::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ReviewInstruction::unpack(instruction_data)?;
    match instruction {
        ReviewInstruction::AddReview {
            title,
            rating,
            description,
            location,
        } => add_review(program_id, accounts, title, rating, description, location),
        ReviewInstruction::UpdateReview {
            title,
            rating,
            description,
            location,
        } => update_review(program_id, accounts, title, rating, description, location),
    }
}

pub fn add_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
    location: String,
) -> ProgramResult {
    msg!("Adding  review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);
    msg!("Location: {}", location);
    msg!("Review Added successfully");

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    if rating > 5 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    }

    let account_len: usize = 1000;

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    msg!("checking if  account is already initialized");
    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.location = location;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}

pub fn update_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _title: String,
    rating: u8,
    description: String,
    location: String,
) -> ProgramResult {
    msg!("Updating  review...");

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();
    msg!("review title: {}", account_data.title);

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[
            initializer.key.as_ref(),
            account_data.title.as_bytes().as_ref(),
        ],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into());
    }

    msg!("checking if  account is initialized");
    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }

    if rating > 5 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    }

    msg!("Review before update:");
    msg!("Title: {}", account_data.title);
    msg!("Rating: {}", account_data.rating);
    msg!("Description: {}", account_data.description);
    msg!("Location: {}", account_data.location);

    account_data.rating = rating;
    account_data.description = description;
    account_data.location = location;

    msg!("Review after update:");
    msg!("Title: {}", account_data.title);
    msg!("Rating: {}", account_data.rating);
    msg!("Description: {}", account_data.description);
    msg!("Location: {}", account_data.location);

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use solana_program::pubkey::Pubkey;
    use solana_program::rent::Rent;
    use solana_program::system_program;

    // Helper function to create dummy AccountInfo
    fn create_account_info<'a>(
        key: &'a Pubkey,
        is_signer: bool,
        is_writable: bool,
        lamports: &'a mut u64,
        data: &'a mut [u8],
        owner: &'a Pubkey,
    ) -> AccountInfo<'a> {
        AccountInfo::new(
            key,
            is_signer,
            is_writable,
            lamports,
            data,
            owner,
            false,
            Epoch::default(),
        )
    }

    #[test]
    fn test_process_instruction_add_review() {
        let program_id = Pubkey::new_unique();
        let user_key = Pubkey::new_unique();
        let (pda, _) = Pubkey::find_program_address(&[user_key.as_ref(), b"Some title"], &program_id);

        let mut user_account = user_key;
        let mut lamports = 100000;
        let mut data = vec![0; 1000];
        
        let user_account_info = create_account_info(
            &user_account,
            true,
            false,
            &mut lamports,
            &mut data,
            &system_program::id(),
        );

        let mut pda_account = pda;
        let mut pda_lamports = 0;
        let mut pda_data = vec![0; 1000];
        
        let pda_account_info = create_account_info(
            &pda_account,
            false,
            true,
            &mut pda_lamports,
            &mut pda_data,
            &program_id,
        );

        let system_program_account_info = create_account_info(
            &system_program::id(),
            false,
            false,
            &mut 0,
            &mut [],
            &system_program::id(),
        );

        let accounts = vec![
            user_account_info,
            pda_account_info,
            system_program_account_info,
        ];

        let instruction_data = ReviewInstruction::AddReview {
            title: "Some title".to_string(),
            rating: 5,
            description: "Great!".to_string(),
            location: "Somewhere".to_string(),
        }
        .try_to_vec()
        .unwrap();

        let result = process_instruction(&program_id, &accounts, &instruction_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_instruction_update_review() {
        let program_id = Pubkey::new_unique();
        let user_key = Pubkey::new_unique();
        let (pda, _) = Pubkey::find_program_address(&[user_key.as_ref(), b"Some title"], &program_id);

        let mut user_account = user_key;
        let mut lamports = 100000;
        let mut data = vec![0; 1000];
        
        let user_account_info = create_account_info(
            &user_account,
            true,
            false,
            &mut lamports,
            &mut data,
            &system_program::id(),
        );

        let mut pda_account = pda;
        let mut pda_lamports = 0;
        let mut pda_data = AccountState {
            title: "Some title".to_string(),
            rating: 4,
            description: "Good".to_string(),
            location: "Somewhere".to_string(),
            is_initialized: true,
        }
        .try_to_vec()
        .unwrap();
        
        let pda_account_info = create_account_info(
            &pda_account,
            false,
            true,
            &mut pda_lamports,
            &mut pda_data,
            &program_id,
        );

        let accounts = vec![user_account_info, pda_account_info];

        let instruction_data = ReviewInstruction::UpdateReview {
            title: "Some title".to_string(),
            rating: 5,
            description: "Great!".to_string(),
            location: "Somewhere else".to_string(),
        }
        .try_to_vec()
        .unwrap();

        let result = process_instruction(&program_id, &accounts, &instruction_data);
        assert!(result.is_ok());
    }

    // Add more tests for error cases...
}
