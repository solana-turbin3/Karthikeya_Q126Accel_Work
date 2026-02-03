#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use spl_transfer_hook_interface::instruction::ExecuteInstruction;

mod instructions;
mod state;

use instructions::*;

declare_id!("6SNMfvWd447DkTb9Md7NonphbNNtMAayxFt2qSZpMk2W");

#[program]
pub mod whitelisted_transfers {
    use spl_tlv_account_resolution::state::ExtraAccountMetaList;

    use super::*;
    pub fn whitelist_user(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.whitelist_user(user)?;
        Ok(())
    }

    pub fn blacklist_user(ctx: Context<WhitelistOperations>, user: Pubkey) -> Result<()> {
        ctx.accounts.blacklist_user(user)?;
        Ok(())
    }

    pub fn init_transfer_hook(ctx: Context<InitExtraAccMeta>) -> Result<()> {
        msg!("Initializing Extra Account Meta List");
        
        let extra_acc_metas = InitExtraAccMeta::extra_account_metas()?;
        
        msg!("Extra Account Metas: {:?}", extra_acc_metas);
        msg!("Extra Account Metas Length: {}", extra_acc_metas.len());

        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_acc_meta_list.try_borrow_mut_data()?,
            &extra_acc_metas,
        )
        .unwrap();
        Ok(())
    }
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(amount)?;
        Ok(())
    }
}
