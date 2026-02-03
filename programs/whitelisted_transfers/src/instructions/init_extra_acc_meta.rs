use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};

#[derive(Accounts)]
pub struct InitExtraAccMeta<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
    init,
    space = ExtraAccountMetaList::size_of(
            InitExtraAccMeta::extra_account_metas()?.len()
        ).unwrap(),
        payer = signer,
    seeds = [b"extra-account-metas",mint.key().as_ref()],
    bump
  )]
    pub extra_acc_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitExtraAccMeta<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        let signer_status_pda = ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal {
                    bytes: b"whitelist".to_vec(),
                },
                Seed::AccountKey { index: 1 }, // index 1= destination
            ],
            false, // is_signer
            true,  // is_writable
        )
        .unwrap();
        let account_metas = vec![signer_status_pda];

        Ok(account_metas)
    }
}
