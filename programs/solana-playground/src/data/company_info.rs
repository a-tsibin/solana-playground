use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default)]
pub struct CompanyInfo {
    pub id: u16,
    pub donations_sum: u64,
    pub withdrawn_sum: u64,
}

impl CompanyInfo {
    pub const SIZE: usize = 2 + 8 + 8;
}
