use anchor_lang::prelude::*;

declare_id!("FYJwg6zgrXCqMuEM5Nak8EMdCXyfDirYSkcvcYJZaxBT");

#[program]
pub mod first_nft {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
