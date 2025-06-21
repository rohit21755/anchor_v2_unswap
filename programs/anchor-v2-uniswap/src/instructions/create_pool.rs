use anchor_lang::prelude::*;

use anchor_spl::token::{self, Mint, Token};
use anchor_spl::associated_token::AssociatedToken;

use crate::{
    constants::*,
    errors::*,
    state::{Amm, Pool},
};

pub fn create_pool(ctx: Context<CreatePool>, mint_a: Pubkey, mint_b: Pubkey) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.amm = ctx.accounts.amm.key();
    pool.mint_a = mint_a;
    pool.mint_b = mint_b;
    msg!(
        "Pool created with AMM: {:?}, Mint A: {:?}, Mint B: {:?}",
        pool.amm,
        pool.mint_a,
        pool.mint_b
    );
    Ok(())
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
     #[account(
        seeds = [
            amm.id.as_ref()
        ],
        bump,
    )]
    pub amm: Account<'info, Amm>,

    #[account(
        init,
        payer = payer,
        space = 8 + Pool::INIT_SPACE,
        seeds = [
            amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
        ],
        bump,
        constraint = mint_a.key() < mint_b.key() @ TutorialError::InvalidMint
    )]
    pub pool: Account<'info, Pool>,

    /// CHECK: Read only authority
    #[account(
        seeds = [
            b"authority",
            amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
           
        ],
        bump,
    )]
    pub pool_authority: AccountInfo<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [
             b"liquidity",
            amm.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
           
        ],
        bump,
        mint::decimals = 6,
        mint::authority = pool_authority,
    )]
    pub mint_liquidity: Account<'info, Mint>,

    pub mint_a: Account<'info, Mint>,

    pub mint_b: Account<'info, Mint>,

    /// The account paying for all rents
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Solana ecosystem accounts
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}