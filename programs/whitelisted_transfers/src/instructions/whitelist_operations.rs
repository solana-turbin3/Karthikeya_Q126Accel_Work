use anchor_lang::prelude::*;

use crate::state::Status;

#[derive(Accounts)]
#[instruction(user_ata: Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
    init_if_needed,
    payer = admin,
    space = 8 + 1 ,
    seeds = [b"whitelist-status",user_ata.as_ref()] ,
    bump,
  )]
    pub whitelist_status: Account<'info, Status>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn whitelist_user(&mut self, user_ata: Pubkey) -> Result<()> {
        msg!("user ata {} whitelisted", user_ata);
        self.whitelist_status.set_inner(Status {
            whitelist_status: true,
        });
        Ok(())
    }

    pub fn blacklist_user(&mut self, user_ata: Pubkey) -> Result<()> {
        msg!("user ata {} balcklisted", user_ata);
        self.whitelist_status.set_inner(Status {
            whitelist_status: false,
        });
        Ok(())
    }
}
