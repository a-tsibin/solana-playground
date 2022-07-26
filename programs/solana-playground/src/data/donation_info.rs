use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default)]
pub struct DonationInfo {
    pub donor: Pubkey,
    pub amount: u64,
}

impl DonationInfo {
    pub const SIZE: usize = 32 + 8;
}
