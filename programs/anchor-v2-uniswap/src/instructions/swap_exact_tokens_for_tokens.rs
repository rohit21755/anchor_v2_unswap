use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer},
    token,
};
use fixed::types::I64F64;
use crate::{
    errors::TutorialError,
    state::{Amm, Pool},
};


pub fn swap_exact_tokens_for_tokens(
    ctx: Context<SwapExactTokensForTokens>,
    swap_a: bool,
    input_amount: u64,
    min_output_amount: u64,
) ->Result<()> {
    let input = if swap_a && input_amount > ctx.accounts.trader_account_a.amount {
        ctx.accounts.trader_account_a.amount
    } else if !swap_a && input_amount > ctx.accounts.trader_account_b.amount {
        ctx.accounts.trader_account_b.amount
    } else {
        input_amount
    };

    let amm = &ctx.accounts.amm;
    let taxed_input = input -input * amm.fee as u64 /1000;

    let pool_a = &ctx.accounts.pool_account_a;
    let pool_b = &ctx.accounts.pool_account_b;
      let output = if swap_a {
            I64F64::from_num(taxed_input)
                .checked_mul(I64F64::from_num(pool_b.amount))
                .unwrap()
                .checked_div(
                    I64F64::from_num(pool_a.amount)
                        .checked_add(I64F64::from_num(taxed_input))
                        .unwrap(),
                )
                .unwrap()
        } else {
            I64F64::from_num(taxed_input)
                .checked_mul(I64F64::from_num(pool_a.amount))
                .unwrap()
                .checked_div(
                    I64F64::from_num(pool_b.amount)
                        .checked_add(I64F64::from_num(taxed_input))
                        .unwrap(),
                )
                .unwrap()
        }
        .to_num::<u64>();

    if output < min_output_amount {
        return Err(TutorialError::OutputTooSmall.into());
    }
    let authority_bump = ctx.bumps.pool_authority;
    let mint_a_key = ctx.accounts.mint_a.key();
    let mint_b_key = ctx.accounts.mint_b.key();
    let invariant = pool_a.amount * pool_b.amount;
    let signer_seeds: &[&[&[u8]]] = &[&[
            b"authority",
            ctx.accounts.pool.amm.as_ref(),
            mint_a_key.as_ref(),
            mint_b_key.as_ref(),
            &[authority_bump],
        ]];

    if swap_a {
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer { from: ctx.accounts.trader_account_a.to_account_info(), to: ctx.accounts.pool_account_a.to_account_info(), authority: ctx.accounts.trader.to_account_info() }), input)?;
        token::transfer(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), Transfer { from: ctx.accounts.pool_account_b.to_account_info(), to: ctx.accounts.trader_account_b.to_account_info(), authority: ctx.accounts.pool_authority.to_account_info() }, signer_seeds), output)?;
    } else {
        token::transfer(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), Transfer { from: ctx.accounts.pool_account_a.to_account_info(), to: ctx.accounts.trader_account_a.to_account_info(), authority: ctx.accounts.pool_authority.to_account_info() }, signer_seeds), output)?;
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer { from: ctx.accounts.trader_account_b.to_account_info(), to: ctx.accounts.pool_account_b.to_account_info(), authority: ctx.accounts.trader.to_account_info() }), input)?;
    }
     msg!(
            "Traded {} tokens ({} after fees) for {}",
            input,
            taxed_input,
            output
        );

        // Verify the invariant still holds
        // Reload accounts because of the CPIs
        // We tolerate if the new invariant is higher because it means a rounding error for LPs
        ctx.accounts.pool_account_a.reload()?;
        ctx.accounts.pool_account_b.reload()?;
        if invariant > ctx.accounts.pool_account_a.amount * ctx.accounts.pool_account_b.amount {
            return err!(TutorialError::InvariantViolated);
        }

        Ok(())
}


#[derive(Accounts)]
pub struct SwapExactTokensForTokens<'info>{
    #[account(
        seeds = [amm.id.as_ref()],
        bump,
    )]
    pub amm: Account<'info, Amm>,
    
    #[account(
        seeds = [
            pool.amm.as_ref(),
            pool.mint_a.as_ref(),
            pool.mint_b.as_ref(),
        ],
        bump,
        has_one = amm,
        has_one = mint_a,
        has_one = mint_b,
    )]
    pub pool: Account<'info, Pool>,
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

    pub mint_a: Box<Account<'info, Mint>>,

    pub mint_b: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub trader: Signer<'info>,

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
        payer = trader,
        associated_token::mint = mint_a,
        associated_token::authority = trader,
    )]
    pub trader_account_a: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = trader,
        associated_token::mint = mint_b,
        associated_token::authority = trader,
    )]
    pub trader_account_b: Box<Account<'info, TokenAccount>>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}