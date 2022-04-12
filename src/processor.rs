//processor.rs -> program logic

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::invoke
};

use crate::{instruction::DepositInstruction, error::DepositError, state::Deposit};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = DepositInstruction::unpack(instruction_data)?;

        match instruction {
            DepositInstruction::InitDeposit { amount } => {
                msg!("Instruction: InitDeposit");
                Self::process_init_deposit(accounts, amount, program_id)
            }
        }
    }

    fn process_init_deposit(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let temp_token_account = next_account_info(account_info_iter)?;

        // let token_to_receive_account = next_account_info(account_info_iter)?;
        // if *token_to_receive_account.owner != spl_token::id() {
        // return Err(ProgramError::IncorrectProgramId);
        // }

        let deposit_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(deposit_account.lamports(), deposit_account.data_len()) {
        return Err(DepositError::NotRentExempt.into());
        }

        let mut deposit_info = Deposit::unpack_unchecked(&deposit_account.try_borrow_data()?)?;
        if deposit_info.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
        }

        deposit_info.is_initialized = true;
        deposit_info.initializer_pubkey = *initializer.key;
        deposit_info.temp_token_account_pubkey = *temp_token_account.key;

        Deposit::pack(deposit_info, &mut deposit_account.try_borrow_mut_data()?)?;

        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"deposit"], program_id);

        let token_program = next_account_info(account_info_iter)?;
        //confusion of token ownership transfer
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key, //token program id, 
            temp_token_account.key, //then the account whose authority we'd like to change,
            Some(&pda), // the account that's the new authority (in our case the PDA),
            spl_token::instruction::AuthorityType::AccountOwner, //the type of authority change (there are different authority types for token accounts, we care about changing the main authority),
            initializer.key,        //the current account authority (Alice -> initializer.key),
            &[&initializer.key],    //the public keys signing the CPI.
        )?;
        

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;
        // end of confusion

        Ok(())

        //end of process_init_deposit
    } 
}
