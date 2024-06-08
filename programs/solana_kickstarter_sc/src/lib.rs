use anchor_lang::prelude::*;

declare_id!("4dm6DpP41wiFUgFy8FHrGACtRTF9mBjfTTx3XS2ap8xt");

#[program]
pub mod solana_kickstarter_sc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
