//processor.rs -> program logic

use solana_program::{
    account_info::AccountInfo,  
    entrypoint::ProgramResult,
    msg, 
    pubkey::Pubkey,
};

use crate::instruction::DepositInstruction;
use crate::processor::Processor;


use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};