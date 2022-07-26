use crate::account::donation_company::DonationCompany;
use crate::account::platform_settings::PlatformSettings;
use crate::errors::CustomErrors;
use crate::events::CloseCompanyEvent;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use anchor_spl::token::{self, Burn, CloseAccount, Mint, Token, TokenAccount};

fn close_chrt_vaults(ctx: &Context<CloseCompany>) -> Result<()> {
    let signer: &[&[&[u8]]] = &[&[b"platform_settings", &[ctx.accounts.platform_settings.bump]]];

    for vault in [
        &ctx.accounts.fee_exemption_vault,
        &ctx.accounts.close_token_vault,
    ] {
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.chrt_mint.to_account_info(),
                from: vault.to_account_info(),
                authority: ctx.accounts.platform_settings.to_account_info(),
            },
            signer,
        );
        token::burn(cpi_ctx, vault.amount)?;

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: vault.to_account_info(),
                destination: ctx.accounts.initiator.to_account_info(),
                authority: ctx.accounts.platform_settings.to_account_info(),
            },
            signer,
        );
        token::close_account(cpi_ctx)?;
    }

    Ok(())
}

fn withdraw_lamports(ctx: &Context<CloseCompany>, lamports: u64) -> Result<()> {
    invoke_signed(
        &system_instruction::transfer(
            ctx.accounts.sol_wallet.key,
            ctx.accounts.initiator.key,
            lamports,
        ),
        &[
            ctx.accounts.sol_wallet.to_account_info(),
            ctx.accounts.initiator.to_account_info(),
        ],
        &[&[
            b"sol_wallet",
            &[ctx.accounts.platform_settings.bump_sol_wallet],
        ]],
    )?;
    Ok(())
}

pub fn close_company(ctx: Context<CloseCompany>) -> Result<()> {
    close_chrt_vaults(&ctx)?;

    let i = (ctx.accounts.platform_settings.active_companies)
        .binary_search_by_key(&ctx.accounts.donation_company.id, |c| c.id)
        .map_err(|_| CustomErrors::CompanyByIndexError)?;
    let company = ctx.accounts.platform_settings.active_companies.remove(i);

    withdraw_lamports(&ctx, company.donations_sum - company.withdrawn_sum)?;

    emit!(CloseCompanyEvent {
        company_id: company.id
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CloseCompany<'info> {
    #[account(mut, seeds = [b"platform_settings"], bump = platform_settings.bump)]
    platform_settings: Account<'info, PlatformSettings>,
    /// CHECK:
    #[account(mut, seeds = [b"sol_wallet"], bump = platform_settings.bump_sol_wallet)]
    sol_wallet: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"chrt_mint"], bump = platform_settings.bump_mint)]
    chrt_mint: Account<'info, Mint>,
    #[account(
        mut,
        close = initiator,
        seeds = [b"donation_company", donation_company.id.to_le_bytes().as_ref()],
        bump = donation_company.bump,
    )]
    donation_company: Account<'info, DonationCompany>,
    #[account(mut, address = platform_settings.owner_key)]
    initiator: Signer<'info>,
    #[account(
        mut,
        seeds = [b"bump_fee_token_vault", donation_company.id.to_le_bytes().as_ref()],
        bump = donation_company.bump_fee_token_vault,
    )]
    fee_exemption_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"bump_close_token_vault", donation_company.id.to_le_bytes().as_ref()],
        bump = donation_company.bump_close_token_vault,
    )]
    close_token_vault: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}
