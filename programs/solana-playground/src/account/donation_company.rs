use crate::account::platform_settings::CAMPAIGN_TOP_CAPACITY;
use crate::data::donation_info::DonationInfo;
use anchor_lang::prelude::*;

#[account]
pub struct DonationCompany {
    pub id: u16,
    pub bump: u8,
    pub bump_fee_token_vault: u8,
    pub bump_close_token_vault: u8,
    pub initiator: Pubkey,
    pub top_donation: Vec<DonationInfo>,
}
impl DonationCompany {
    pub const SIZE: usize = 1 + 2 + 1 + 1 + 32 + (4 + CAMPAIGN_TOP_CAPACITY * DonationInfo::SIZE);
}
