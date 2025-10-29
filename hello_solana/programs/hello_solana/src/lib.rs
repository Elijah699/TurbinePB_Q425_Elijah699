use anchor_lang::prelude::*;

declare_id!("72KsAPvShMgAEJYoe45bR2CXuonTjiH2z69Uf4dHVTeF");

#[program]
pub mod anchor_setup {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello Solana!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
