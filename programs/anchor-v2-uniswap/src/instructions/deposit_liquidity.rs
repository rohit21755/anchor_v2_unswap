use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer},
};
use fixed::types::I64F64;

use crate::{
    constants::{ MINIMUM_LIQUIDITY},
    errors::TutorialError,
    state::Pool,
};

 pub fn deposit_liquidity(
        ctx: Context<DepositLiquidity>,
        amount_a: u64,
        amount_b: u64,
    ) -> Result<()> {
        let mut amount_a = if amount_a > ctx.accounts.depositor_account_a.amount  {
            ctx.accounts.depositor_account_a.amount
        } else {
            amount_a
        };

        let mut amount_b = if amount_b > ctx.accounts.depositor_account_b.amount {
            ctx.accounts.depositor_account_b.amount
        } else {
            amount_b
        };

        let pool_a = &ctx.accounts.pool_account_a;
        let pool_b = &ctx.accounts.pool_account_b;

        let pool_creation = pool_a.amount == 0 && pool_b.amount == 0;
        (amount_a, amount_b) = if pool_creation {
            (amount_a, amount_b)
        } else {
            let ratio = I64F64::from_num(pool_a.amount)
                .checked_div(I64F64::from_num(pool_b.amount))
                .unwrap();
            if pool_a.amount > pool_b.amount {
                (
                    I64F64::from_num(amount_b)
                        .checked_mul(ratio)
                        .unwrap()
                        .to_num::<u64>(),
                    amount_b,
                )
            } else {
                (
                    amount_a,
                    I64F64::from_num(amount_a)
                        .checked_div(ratio)
                        .unwrap()
                        .to_num::<u64>(),
                )
            }
        };

        let mut liquidity = I64F64::from_num(amount_a)
            .checked_mul(I64F64::from_num(amount_b))
            .unwrap()
            .sqrt()
            .to_num::<u64>();

        if pool_creation {
            if liquidity < MINIMUM_LIQUIDITY {
                return Err(TutorialError::DepositTooSmall.into());
            }
            liquidity = liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap();
        }

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.depositor_account_a.to_account_info(),
            to: pool_a.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);
        token::transfer(cpi_ctx, amount_a)?;
        let cpi_accounts = Transfer {
            from: ctx.accounts.depositor_account_b.to_account_info(),
            to: pool_b.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program.clone(), cpi_accounts);
        token::transfer(cpi_ctx, amount_b)?;

        // mint liquidity tokens
        let authority_bump = ctx.bumps.pool_authority;
        let mint_a_key = ctx.accounts.mint_a.key();
        let mint_b_key = ctx.accounts.mint_b.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"authority",
            ctx.accounts.pool.amm.as_ref(),
            mint_a_key.as_ref(),
            mint_b_key.as_ref(),
            &[authority_bump],
        ]];
        let cpi_accounts_mint = MintTo {
            mint: ctx.accounts.mint_liquidity.to_account_info(),
            to: ctx.accounts.depositor_account_liquidity.to_account_info(),
            authority: ctx.accounts.pool_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts_mint)
            .with_signer(signer_seeds);
        token::mint_to(cpi_ctx, liquidity)?;
        Ok(())
    }

#[derive(Accounts)]
pub struct DepositLiquidity<'info> {
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

    pub mint_a: Box<Account<'info, Mint>>,

    pub mint_b: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_a,
        associated_token::authority = pool_authority,
    )]
    pub pool_account_a: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
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