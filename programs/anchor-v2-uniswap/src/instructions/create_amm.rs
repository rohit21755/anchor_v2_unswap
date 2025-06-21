use anchor_lang::prelude::*;
use crate::{errors::*, state::Amm};


pub fn create_amm(ctx: Context<CreateAmm>, id:Pubkey, fee: u16) -> Result<()>{
    let amm = &mut ctx.accounts.amm;
    amm.id = id;
    amm.admin = ctx.accounts.admin.key();
    amm.fee = fee;
    msg!("Amm created with ID: {:?}, Admin: {:?}, Fee: {}", amm.id, amm.admin, amm.fee);
    Ok(())
}
#[derive(Accounts)]
#[instruction(id: Pubkey, fee: u16)]
pub struct CreateAmm<'info> {
    #[account(
        init,
        payer = payer,
        space = Amm::INIT_SPACE,
        seeds = [
            id.as_ref()
        ],
        bump,
        constraint = fee < 10000 @ TutorialError::InvalidFee,
    )]
    pub amm: Account<'info, Amm>,
    /// CHECK: This is the admin address; we don't need to validate it because [EXPLAIN WHY]
    pub admin: AccountInfo<'info>, // this is the admin address, we don't need to validate it because it's just a reference to the admin's public key, which is validated in the create_amm function
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}