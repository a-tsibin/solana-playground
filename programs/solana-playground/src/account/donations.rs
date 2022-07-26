use anchor_lang::prelude::*;

#[account]
pub struct Donations {
    pub bump: u8,
    pub donations_sum: u64,
}
impl Donations {
    pub const SIZE: usize = 1 + 8;
}
