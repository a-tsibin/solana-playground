use crate::data::company_info::CompanyInfo;
use crate::data::donation_info::DonationInfo;
use anchor_lang::prelude::*;

pub const PLATFORM_TOP_CAPACITY: usize = 100;
pub const CAMPAIGN_TOP_CAPACITY: usize = 10;

#[account]
pub struct PlatformSettings {
    pub owner_key: Pubkey,
    pub bump: u8,
    pub bump_fee_wallet: u8,
    pub bump_sol_wallet: u8,
    pub bump_mint: u8,
    pub campaigns_capacity: u16,
    pub fee_num: u64,
    pub fee_denom: u64,
    pub fee_free_token_amount: u64,
    pub close_company_token_amount: u64,
    pub token_drop_amount: u64,
    pub last_token_drop_ts: u64,
    pub min_token_drop_period: u64,
    pub top_donation: Vec<DonationInfo>,
    pub campaigns_count: u16,
    pub active_companies: Vec<CompanyInfo>,
}

impl PlatformSettings {
    pub const fn size(campaigns_capacity: u16) -> usize {
        (32 + 1 + 1 + 1 + 1 + 1 + 2 + 8 + 8 + 8 + 8 + 8 + 8 + 2)
            + (4 + PLATFORM_TOP_CAPACITY * DonationInfo::SIZE)
            + (4 + campaigns_capacity as usize * CompanyInfo::SIZE)
    }
}
