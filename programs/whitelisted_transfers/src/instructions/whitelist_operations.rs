use anchor_lang::prelude::*;

use crate::state::Whitelist;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct WhitelistOperations<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
    init_if_needed,
    payer = admin,
    space = 8 + 1 ,
    seeds = [b"whitelist-status",user.as_ref()] ,
    bump,
  )]
    pub whitelist_status: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn whitelist_user(&mut self, user: Pubkey) -> Result<()> {
        msg!("user {} whitelisted", user);
        self.whitelist_status.set_inner(Whitelist {
            whitelist_status: true,
        });
        Ok(())
    }

    pub fn blacklist_user(&mut self, user: Pubkey) -> Result<()> {
        msg!("user {} balcklisted", user);
        self.whitelist_status.set_inner(Whitelist {
            whitelist_status: false,
        });
        Ok(())
    }
}
