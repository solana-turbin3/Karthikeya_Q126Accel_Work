#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use spl_transfer_hook_interface::instruction::{ExecuteInstruction, TransferHookInstruction};

mod instructions;
mod state;

use instructions::*;

declare_id!("6SNMfvWd447DkTb9Md7NonphbNNtMAayxFt2qSZpMk2W");

#[program]
pub mod whitelisted_transfers {
    use super::*;

    pub fn initialize_extra_account_meta_list(ctx: Context<InitExtraAccMeta>) -> Result<()> {
        Ok(())
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        data: &[u8],
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferHook {}
