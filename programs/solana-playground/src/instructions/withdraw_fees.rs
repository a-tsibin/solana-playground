use crate::account::platform_settings::PlatformSettings;
use crate::events::WithdrawFeesEvent;
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
};

pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
    invoke_signed(
        &system_instruction::transfer(
            ctx.accounts.fee_wallet.key,
            ctx.accounts.owner.key,
            ctx.accounts.fee_wallet.lamports() - Rent::get()?.minimum_balance(0),
        ),
        &[
            ctx.accounts.fee_wallet.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ],
        &[&[
            b"fee_wallet",
            &[ctx.accounts.platform_settings.bump_fee_wallet],
        ]],
    )?;

    emit!(WithdrawFeesEvent {});

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(seeds = [b"platform_settings"], bump = platform_settings.bump)]
    platform_settings: Account<'info, PlatformSettings>,
    #[account(mut, address = platform_settings.owner_key)]
    owner: Signer<'info>,
    /// CHECK:
    #[account(mut, seeds = [b"fee_wallet"], bump = platform_settings.bump_fee_wallet)]
    fee_wallet: AccountInfo<'info>,
    system_program: Program<'info, System>,
}
