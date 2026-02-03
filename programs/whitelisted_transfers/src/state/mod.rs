use anchor_lang::prelude::*;

#[account]
pub struct Status {
    pub whitelist_status: bool,
}
