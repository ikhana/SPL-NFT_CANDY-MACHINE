use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
declare_id!("FYJwg6zgrXCqMuEM5Nak8EMdCXyfDirYSkcvcYJZaxBT");

#[program]
pub mod first_nft {
    use super::*;

    pub fn initialize(_ctx: Context<InitNFT>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]

pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing in this account ourselves
    #[account(mut, signer)]
   pub signer: AccountInfo<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
  pub mint: Account<'info, Mint>, // new
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub associated_token_account: Account<'info, TokenAccount>, // new

    pub token_program: Program<'info, Token>, // new
    pub associated_token_program: Program<'info, AssociatedToken>, // new
    pub system_program: Program<'info, System>, // bew
    pub rent: Sysvar<'info, Rent> // new
}
pub struct Initialize {}
