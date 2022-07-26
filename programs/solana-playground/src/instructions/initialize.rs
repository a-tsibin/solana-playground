use crate::account::platform_settings::PlatformSettings;
use crate::errors::CustomErrors;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

pub fn initialize(
    ctx: Context<Initialize>,
    campaigns_capacity: u16,
    fee_num: u64,
    fee_denom: u64,
    fee_free_token_amount: u64,
    token_drop_amount: u64,
    token_drop_period: u64,
    close_company_token_amount: u64,
) -> Result<()> {
    msg!("Start initializing platform");
    ctx.accounts.platform_settings.owner_key = *ctx.accounts.owner.key;
    ctx.accounts.platform_settings.campaigns_capacity = campaigns_capacity;
    ctx.accounts.platform_settings.bump = *ctx
        .bumps
        .get("platform_settings")
        .ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.platform_settings.bump_fee_wallet =
        *ctx.bumps.get("fee_wallet").ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.platform_settings.bump_sol_wallet =
        *ctx.bumps.get("sol_vault").ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.platform_settings.bump_mint =
        *ctx.bumps.get("chrt_mint").ok_or(CustomErrors::EmptyBump)?;
    if fee_num >= fee_denom {
        return err!(CustomErrors::InvalidFeeValue);
    }
    ctx.accounts.platform_settings.fee_num = fee_num;
    ctx.accounts.platform_settings.fee_denom = fee_denom;
    ctx.accounts.platform_settings.token_drop_amount = token_drop_amount;
    ctx.accounts.platform_settings.min_token_drop_period = token_drop_period;
    ctx.accounts.platform_settings.fee_free_token_amount = fee_free_token_amount;
    ctx.accounts.platform_settings.close_company_token_amount = close_company_token_amount;
    msg!("Platform initialized");
    Ok(())
}

#[derive(Accounts)]
#[instruction(campaigns_capacity: u16)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + PlatformSettings::size(campaigns_capacity),
        payer = owner,
        seeds = [b"platform_settings"],
        bump
    )]
    pub platform_settings: Account<'info, PlatformSettings>,
    /// CHECK:
    #[account(
        init,
        space = 0,
        payer = owner,
        seeds = [b"fee_wallet"],
        bump,
        owner = system_program.key(),
    )]
    pub fee_wallet: AccountInfo<'info>,
    /// CHECK:
    #[account(
        init,
        payer = owner,
        seeds = [b"sol_wallet"],
        bump,
        space = 0,
        owner = system_program.key(),
    )]
    sol_wallet: UncheckedAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"chrt_mint"],
        bump,
        mint::decimals = 3,
        mint::authority = platform_settings,
    )]
    pub chrt_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}
