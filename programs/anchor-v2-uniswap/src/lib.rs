use anchor_lang::prelude::*;
mod state;
mod constants;
mod errors;
declare_id!("EWG9BxbMHbCdqsVeu7mKcPua7VetUy2adZciWKonZKgy");

#[program]
pub mod anchor_v2_uniswap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
