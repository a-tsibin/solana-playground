use crate::account::donator::Donator;
use crate::account::platform_settings::PlatformSettings;
use crate::errors::CustomErrors;
use crate::events::TokenDropEvent;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount};

const TOP_CAPACITY: usize = 10;

fn mint_chrt<'info>(
    ctx: &Context<'_, '_, '_, 'info, DropTokens<'info>>,
    donator_chrt: &Account<'info, TokenAccount>,
    amount: u64,
) -> Result<()> {
    let signer: &[&[&[u8]]] = &[&[b"platform", &[ctx.accounts.platform_settings.bump]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.chrt_mint.to_account_info(),
            to: donator_chrt.to_account_info(),
            authority: ctx.accounts.platform_settings.to_account_info(),
        },
        signer,
    );

    emit!(TokenDropEvent {
        to: donator_chrt.key(),
        amount: amount
    });

    token::mint_to(cpi_ctx, amount)
}

pub fn drop_tokens<'info>(ctx: Context<'_, '_, '_, 'info, DropTokens<'info>>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp as _;

    if now - ctx.accounts.platform_settings.last_token_drop_ts
        < ctx.accounts.platform_settings.min_token_drop_period
    {
        return err!(CustomErrors::TooEarlyForTokenDrop);
    }
    ctx.accounts.platform_settings.last_token_drop_ts = now;

    let mut prev_donors = Vec::with_capacity(10);

    for pair in (ctx.remaining_accounts).chunks_exact(2).take(TOP_CAPACITY) {
        let mut donator = Account::<Donator>::try_from(&pair[0])?;
        if prev_donors.contains(&donator.key()) {
            return err!(CustomErrors::DuplicateInTop);
        }
        prev_donors.push(donator.key());
        donator.token_drop_sum = donator.donations_sum;
        donator.try_serialize(&mut &mut pair[0].try_borrow_mut_data()?[..])?;

        let donor_chrt = Account::<TokenAccount>::try_from(&pair[1])?;
        if donor_chrt.owner != donator.authority {
            return err!(CustomErrors::IllegalTokenVaultOwner);
        }

        mint_chrt(
            &ctx,
            &donor_chrt,
            ctx.accounts.platform_settings.token_drop_amount,
        )?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct DropTokens<'info> {
    #[account(mut, seeds = [b"platform_settings"], bump = platform_settings.bump)]
    platform_settings: Account<'info, PlatformSettings>,
    #[account(address = platform_settings.owner_key)]
    owner: Signer<'info>,
    #[account(mut, seeds = [b"chrt_mint"], bump = platform_settings.bump_mint)]
    chrt_mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
}
