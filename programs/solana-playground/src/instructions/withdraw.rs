use crate::account::donation_company::DonationCompany;
use crate::account::platform_settings::PlatformSettings;
use crate::errors::CustomErrors;
use crate::events::WithdrawEvent;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

fn transfer_lamports(ctx: &Context<Withdraw>, amount: u64) -> Result<()> {
    invoke_signed(
        &system_instruction::transfer(
            ctx.accounts.sol_wallet.key,
            ctx.accounts.initiator.key,
            amount,
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

pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let i = (ctx.accounts.platform_settings.active_companies)
        .binary_search_by_key(&ctx.accounts.donation_company.id, |c| c.id)
        .map_err(|_| CustomErrors::CompanyByIndexError)?;
    let lamports = {
        let mut company = ctx
            .accounts
            .platform_settings
            .active_companies
            .get_mut(i)
            .ok_or(CustomErrors::CompanyByIndexError)?;
        let lamports = company.donations_sum - company.withdrawn_sum;
        company.withdrawn_sum = company.donations_sum;
        lamports
    };

    transfer_lamports(&ctx, lamports)?;

    emit!(WithdrawEvent {
        company_id: ctx.accounts.donation_company.id,
        amount: lamports
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"platform_settings"], bump = platform_settings.bump)]
    platform_settings: Account<'info, PlatformSettings>,
    /// CHECK:
    #[account(mut, seeds = [b"sol_wallet"], bump = platform_settings.bump_sol_wallet)]
    sol_wallet: AccountInfo<'info>,
    #[account(
        seeds = [b"donation_company", donation_company.id.to_le_bytes().as_ref()],
        bump = donation_company.bump,
    )]
    donation_company: Account<'info, DonationCompany>,
    #[account(mut, address = donation_company.initiator)]
    initiator: Signer<'info>,
    system_program: Program<'info, System>,
}
