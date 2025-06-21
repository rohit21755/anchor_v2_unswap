use anchor_lang::prelude::*;
pub mod state;
pub mod constants;
pub mod errors;
pub mod instructions;

pub use instructions::*;
declare_id!("EWG9BxbMHbCdqsVeu7mKcPua7VetUy2adZciWKonZKgy");

#[program]
pub mod anchor_v2_uniswap {
    use super::*;

    pub fn initialize_amm(ctx: Context<CreateAmm>, id: Pubkey, fee: u16) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Initializing AMM with ID: {:?} and Fee: {}", id, fee);
        create_amm::create_amm(ctx, id, fee)?;
        Ok(())
    }

    pub fn initialize_pool(ctx: Context<CreatePool>, mint_a: Pubkey, mint_b: Pubkey) -> Result<()> {
        msg!("Initializing Pool");
        create_pool::create_pool(ctx, mint_a, mint_b)?;
        Ok(())
    }
pub fn add_liquidity(
    ctx: Context<DepositLiquidity>,
    amount_a: u64,
    amount_b: u64,
) -> Result<()> {
    msg!("Adding Liquidity");
    
    deposit_liquidity::deposit_liquidity(ctx, amount_a, amount_b)?;
    Ok(())
}

pub fn swap_exact_tokens_for_tokens(
    ctx: Context<SwapExactTokensForTokens>,
    swap_a: bool,
    input_amount: u64,
    min_output_amount: u64,
) -> Result<()> {
    msg!("Swapping Exact Tokens for Tokens");
    
    swap_exact_tokens_for_tokens::swap_exact_tokens_for_tokens(ctx, swap_a, input_amount, min_output_amount)?;
    Ok(())
}
pub fn withdraw_liquidity(
    ctx: Context<WithdrawLiquidity>,
    amount: u64,
) -> Result<()> {
    msg!("Withdrawing Liquidity");
    
    withdraw_liquidity::withdraw_liquidity(ctx, amount)?;
    Ok(())
}
}

