//processor.rs -> program logic

use solana_program::{
    account_info::AccountInfo,  
    entrypoint::ProgramResult,
    msg, 
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
};

use crate::{instruction::EscrowInstruction, error::EscrowError, state::Escrow};

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
        deposit_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        deposit_info.expected_amount = amount;

        Deposit::pack(deposit_info, &mut deposit_account.try_borrow_mut_data()?)?;

        Ok(())
    }
    
}