use anchor_lang::prelude::*;

#[account]
pub struct Donator {
    pub bump: u8,
    pub authority: Pubkey,
    pub donations_sum: u64,
    pub token_drop_sum: u64,
}

impl Donator {
    pub const SIZE: usize = 1 + 32 + 8 + 8;
}
