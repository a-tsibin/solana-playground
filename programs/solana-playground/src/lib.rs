pub mod account;
pub mod data;
pub mod errors;
pub mod events;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("HJXBag2oEJ1dAAgqauLAkRywTc1rpsKgpBQgeMqTuNGa");

#[program]
pub mod donate_solana_v2 {
    use super::*;

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
        instructions::initialize(
            ctx,
            campaigns_capacity,
            fee_num,
            fee_denom,
            fee_free_token_amount,
            token_drop_amount,
            token_drop_period,
            close_company_token_amount,
        )
    }

    pub fn start_donation_company(ctx: Context<InitializeCompany>) -> Result<()> {
        instructions::start_donation_company(ctx)
    }

    pub fn donate(ctx: Context<MakeDonation>, amount: u64) -> Result<()> {
        instructions::donate(ctx, amount)
    }

    pub fn donate_with_referer(ctx: Context<MakeDonationWithReferer>, amount: u64) -> Result<()> {
        instructions::donate_with_referer(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::withdraw(ctx)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        withdraw_fees::withdraw_fees(ctx)
    }

    pub fn close_company(ctx: Context<CloseCompany>) -> Result<()> {
        close_company::close_company(ctx)
    }

    pub fn drop_tokens<'info>(ctx: Context<'_, '_, '_, 'info, DropTokens<'info>>) -> Result<()> {
        instructions::drop_tokens(ctx)
    }
}
