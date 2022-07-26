use crate::account::donation_company::DonationCompany;
use crate::account::donations::Donations;
use crate::account::donator::Donator;
use crate::account::platform_settings::{
    PlatformSettings, CAMPAIGN_TOP_CAPACITY, PLATFORM_TOP_CAPACITY,
};
use crate::data::donation_info::DonationInfo;
use crate::errors::CustomErrors;
use crate::events::DonationEvent;
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};
use anchor_spl::token::{
    self, spl_token::native_mint::DECIMALS, Mint, MintTo, Token, TokenAccount,
};

fn transfer_to_campaign(accounts: &mut MakeDonation, lamports: u64) -> Result<()> {
    let i = (accounts.platform_settings.active_companies)
        .binary_search_by_key(&accounts.donation_company.id, |c| c.id)
        .map_err(|_| CustomErrors::CompanyByIndexError)?;
    accounts.platform_settings.active_companies[i].donations_sum += lamports;
    accounts.donator.donations_sum += lamports;

    invoke(
        &system_instruction::transfer(
            accounts.donator_authority.key,
            accounts.sol_wallet.key,
            lamports,
        ),
        &[
            accounts.donator_authority.to_account_info(),
            accounts.sol_wallet.to_account_info(),
        ],
    )?;
    Ok(())
}

fn transfer_to_platform(accounts: &MakeDonation, lamports: u64) -> Result<()> {
    invoke(
        &system_instruction::transfer(
            accounts.donator_authority.key,
            accounts.fee_wallet.key,
            lamports,
        ),
        &[
            accounts.donator_authority.to_account_info(),
            accounts.fee_wallet.to_account_info(),
        ],
    )?;
    Ok(())
}

fn add_to_top(top: &mut Vec<DonationInfo>, donator_info: DonationInfo, capacity: usize) {
    let cur_i = if let Some(cur_i) = top.iter().position(|d| d.donor == donator_info.donor) {
        top[cur_i] = donator_info;

        cur_i
    } else if top.len() < capacity {
        top.push(donator_info);

        top.len() - 1
    } else {
        let last = top.last_mut().unwrap();
        if last.amount > donator_info.amount {
            return;
        }
        *last = donator_info;

        top.len() - 1
    };

    let new_i = top[..cur_i].partition_point(|d| d.amount >= donator_info.amount);
    top[new_i..=cur_i].rotate_right(1);
}

fn donate_common(accounts: &mut MakeDonation, lamports: u64) -> Result<()> {
    let fee = lamports * accounts.platform_settings.fee_num / accounts.platform_settings.fee_denom;
    if accounts.fee_token_vault.amount < accounts.platform_settings.fee_free_token_amount {
        transfer_to_campaign(accounts, lamports - fee)?;
        transfer_to_platform(accounts, fee)?;
    } else {
        transfer_to_campaign(accounts, lamports)?;
    }

    add_to_top(
        &mut accounts.platform_settings.top_donation,
        DonationInfo {
            donor: accounts.donator_authority.key(),
            amount: accounts.donator.donations_sum,
        },
        PLATFORM_TOP_CAPACITY,
    );
    add_to_top(
        &mut accounts.donation_company.top_donation,
        DonationInfo {
            donor: accounts.donator_authority.key(),
            amount: accounts.total_donations_to_company.donations_sum,
        },
        CAMPAIGN_TOP_CAPACITY,
    );

    Ok(())
}

fn mint_chrt_to_referer(ctx: Context<MakeDonationWithReferer>, amount: u64) -> Result<()> {
    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.donate.platform_settings.bump]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.chrt_mint.to_account_info(),
            to: ctx.accounts.referer_chrt.to_account_info(),
            authority: ctx.accounts.donate.platform_settings.to_account_info(),
        },
        signer,
    );
    token::mint_to(cpi_ctx, amount)
}

pub fn donate(ctx: Context<MakeDonation>, lamports: u64) -> Result<()> {
    let company_id = ctx.accounts.donation_company.id.clone();
    donate_common(ctx.accounts, lamports)?;

    emit!(DonationEvent {
        company_id: company_id,
        amount: lamports
    });

    Ok(())
}

pub fn donate_with_referer(ctx: Context<MakeDonationWithReferer>, lamports: u64) -> Result<()> {
    let company_id = ctx.accounts.donate.donation_company.id.clone();

    donate_common(&mut ctx.accounts.donate, lamports)?;

    mint_chrt_to_referer(ctx, 101 * lamports / 10u64.pow((DECIMALS - 3) as _))?;

    emit!(DonationEvent {
        company_id: company_id,
        amount: lamports
    });

    Ok(())
}

#[derive(Accounts)]
pub struct MakeDonation<'info> {
    #[account(mut, seeds = [b"platform_settings"], bump = platform_settings.bump)]
    platform_settings: Box<Account<'info, PlatformSettings>>,
    /// CHECK:
    #[account(mut, seeds = [b"fee_wallet"], bump = platform_settings.bump_fee_wallet)]
    fee_wallet: AccountInfo<'info>,
    /// CHECK:
    #[account(mut, seeds = [b"sol_wallet"], bump = platform_settings.bump_sol_wallet)]
    sol_wallet: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"donation_company", donation_company.id.to_le_bytes().as_ref()],
        bump = donation_company.bump,
    )]
    donation_company: Account<'info, DonationCompany>,
    #[account(
        mut,
        seeds = [b"donations", donation_company.key().as_ref()],
        bump = total_donations_to_company.bump,
    )]
    total_donations_to_company: Account<'info, Donations>,
    #[account(
        seeds = [b"fee_token_vault", donation_company.id.to_le_bytes().as_ref()],
        bump = donation_company.bump_fee_token_vault,
    )]
    fee_token_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"donator", donator_authority.key().as_ref()],
        bump = donator.bump
    )]
    donator: Account<'info, Donator>,
    #[account(mut)]
    donator_authority: Signer<'info>,
    #[account(
        init_if_needed,
        payer = donator_authority,
        seeds = [b"donations", donator_authority.key().as_ref(), donation_company.key().as_ref()],
        bump,
        space = 8 + Donations::SIZE,
    )]
    donator_donations_to_campaign: Account<'info, Donations>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MakeDonationWithReferer<'info> {
    donate: MakeDonation<'info>,
    #[account(mut, seeds = [b"chrt_mint"], bump = donate.platform_settings.bump_mint)]
    chrt_mint: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"donator", referer_authority.key().as_ref()],
        bump = referer.bump,
        constraint = referer.key() != donate.donator.key() @ CustomErrors::CannotReferYourself,
    )]
    referer: Account<'info, Donator>,
    /// CHECK:
    referer_authority: AccountInfo<'info>,
    #[account(mut, token::authority = referer_authority)]
    referer_chrt: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}
