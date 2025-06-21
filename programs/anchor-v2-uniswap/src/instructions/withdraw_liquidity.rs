use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, mint, token::{self, Mint, Token, TokenAccount, Transfer, Burn}
};

use fixed::types::I64F64;

use crate::{
    constants::{ MINIMUM_LIQUIDITY},
    state::{Amm, Pool},
};

pub fn withdraw_liquidity(
    ctx: Context<WithdrawLiquidity>,
    amount: u64,
) -> Result<()> {
    let mint_a = &ctx.accounts.mint_a.key();
    let mint_b = &ctx.accounts.mint_b.key();
    let singer_seeds: &[&[&[u8]]] = &[&[
        b"authority",
        ctx.accounts.pool.amm.as_ref(),
        mint_a.as_ref(),
        mint_b.as_ref(),
        &[ctx.bumps.pool_authority],
    ]];

    let amount_a = I64F64::from_num(amount)
        .checked_mul(I64F64::from_num(ctx.accounts.pool_account_a.amount))
        .unwrap()
        .checked_div(I64F64::from_num(ctx.accounts.mint_liquidity.supply + MINIMUM_LIQUIDITY))
        .unwrap()
        .floor()
        .to_num();

    token::transfer(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), Transfer { from: ctx.accounts.pool_account_a.to_account_info(), to: ctx.accounts.depositor_account_a.to_account_info(), authority: ctx.accounts.pool_authority.to_account_info() }    , singer_seeds), amount_a)?;

    let amount_b = I64F64::from_num(amount)
            .checked_mul(I64F64::from_num(ctx.accounts.pool_account_b.amount))
            .unwrap()
            .checked_div(I64F64::from_num(
                ctx.accounts.mint_liquidity.supply + MINIMUM_LIQUIDITY,
            ))
            .unwrap()
            .floor()
            .to_num::<u64>();
    
    token::transfer(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), Transfer { from: ctx.accounts.pool_account_b.to_account_info(), to: ctx.accounts.depositor_account_b.to_account_info(), authority: ctx.accounts.pool_authority.to_account_info() }    , singer_seeds), amount_b)?;

     token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.mint_liquidity.to_account_info(),
                    from: ctx.accounts.depositor_account_liquidity.to_account_info(),
                    authority: ctx.accounts.depositor.to_account_info(),
                },
            ),
            amount,
        )?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    #[account(
        seeds = [
            amm.id.as_ref()
        ],
        bump,
    )]
    pub amm: Box<Account<'info, Amm>>,

    #[account(
        seeds = [
            pool.amm.as_ref(),
            pool.mint_a.key().as_ref(),
            pool.mint_b.key().as_ref(),
        ],
        bump,
        has_one = mint_a,
        has_one = mint_b,
    )]
    pub pool: Box<Account<'info, Pool>>,

    /// CHECK: Read only authority
    #[account(
        seeds = [
            b"authority",
            pool.amm.as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),

        ],
        bump,
    )]
    pub pool_authority: AccountInfo<'info>,

    /// The account paying for all rents
    pub depositor: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"liquidity",
            pool.amm.as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),

        ],
        bump,
    )]
    pub mint_liquidity: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub mint_a: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub mint_b: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_b: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_liquidity,
        associated_token::authority = depositor,
    )]
    pub depositor_account_liquidity: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_a,
        associated_token::authority = depositor,
    )]
    pub depositor_account_a: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_b,
        associated_token::authority = depositor,
    )]
    pub depositor_account_b: Box<Account<'info, TokenAccount>>,

    /// The account paying for all rents
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Solana ecosystem accounts
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}