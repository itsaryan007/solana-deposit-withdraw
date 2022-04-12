//processor.rs -> program logic

use crate::{instruction::DepositInstruction, error::DepositError, state::Deposit};
use crate::instruction::DepositInstruction;
use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::DepositError::InvalidInstruction;


use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{instruction::DepositInstruction, error::DepositError, state::Deposit};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = DepositInstruction::unpack(instruction_data)?;

        match instruction {
            DepositInstruction::InitEscrow { amount } => {
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

        Ok(())
    }
}
}

impl DepositInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}