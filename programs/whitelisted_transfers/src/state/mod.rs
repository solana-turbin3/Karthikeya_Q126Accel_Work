use anchor_lang::prelude::*;

#[account]
pub struct Whitelist{
  pub whitelist_status: bool,
}
