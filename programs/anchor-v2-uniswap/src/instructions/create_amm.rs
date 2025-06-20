use anchor_lang::prelude::*;
use crate::{errors::*, state::Amm};

#[derive(Accounts)]
#[instruction()]
pub struct CreateAmm<'info> {
    /// The AMM account to be created
    #[account(
        init,
        payer = admin,
        space = 8 + Amm::INIT_SPACE,
        seeds = [b"amm", admin.key().as_ref()],
        bump
    )]
    pub amm: Account<'info, Amm>,

    /// The admin of the AMM
    #[account(mut)]
    pub admin: Signer<'info>,

    /// System program for account creation
    pub system_program: Program<'info, System>,
}