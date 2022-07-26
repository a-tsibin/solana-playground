use crate::account::donation_company::DonationCompany;
use crate::account::donations::Donations;
use crate::account::platform_settings::PlatformSettings;
use crate::data::company_info::CompanyInfo;
use crate::errors::CustomErrors;
use crate::events::CompanyStartedEvent;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

pub fn start_donation_company(ctx: Context<InitializeCompany>) -> Result<()> {
    msg!("Initializing new donation company");
    if ctx.accounts.platform_settings.campaigns_capacity
        <= ctx.accounts.platform_settings.active_companies.len() as _
    {
        return err!(CustomErrors::CompanyLimit);
    }
    let id = ctx.accounts.platform_settings.campaigns_count;
    ctx.accounts
        .platform_settings
        .active_companies
        .push(CompanyInfo {
            id,
            ..Default::default()
        });
    ctx.accounts.platform_settings.campaigns_count += 1;
    ctx.accounts.donation_company.bump = *ctx
        .bumps
        .get("donation_company")
        .ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.donation_company.bump_fee_token_vault = *ctx
        .bumps
        .get("fee_token_vault")
        .ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.donation_company.bump_close_token_vault = *ctx
        .bumps
        .get("close_token_vault")
        .ok_or(CustomErrors::EmptyBump)?;
    ctx.accounts.total_donations_to_campaign.bump = *ctx
        .bumps
        .get("total_donations_to_campaign")
        .ok_or(CustomErrors::EmptyBump)?;

    ctx.accounts.donation_company.initiator = ctx.accounts.initiator.key();
    msg!("Company initialized");
    emit!(CompanyStartedEvent {
        initiator: ctx.accounts.initiator.key(),
        company_id: id,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCompany<'info> {
    #[account(mut)]
    pub initiator: Signer<'info>,
    #[account(
        init,
        payer = initiator,
        seeds = [b"donation_company", platform_settings.campaigns_count.to_le_bytes().as_ref()],
        bump,
        space = 8 + DonationCompany::SIZE,
    )]
    donation_company: Account<'info, DonationCompany>,
    #[account(
        init,
        payer = initiator,
        seeds = [b"total_donations_to_campaign", donation_company.key().as_ref()],
        bump,
        space = 8 + Donations::SIZE,
    )]
    total_donations_to_campaign: Account<'info, Donations>,
    #[account(seeds = [b"chrt_mint"], bump = platform_settings.bump_mint)]
    pub chrt_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initiator,
        seeds = [b"fee_token_vault"],
        bump,
        token::mint = chrt_mint,
        token::authority = platform_settings
    )]
    pub fee_token_vault: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = initiator,
        seeds = [b"close_token_vault"],
        bump,
        token::mint = chrt_mint,
        token::authority = platform_settings
    )]
    pub close_token_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"platform_settings"],
        bump = platform_settings.bump
    )]
    pub platform_settings: Account<'info, PlatformSettings>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
