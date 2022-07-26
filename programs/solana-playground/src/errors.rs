use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Can't get bump")]
    EmptyBump,
    #[msg("Platform active companies limit reached")]
    CompanyLimit,
    #[msg("The company you are trying to donate is closed")]
    DonationToClosedCompany,
    #[msg("Insufficient token amount")]
    InsufficientTokenAmount,
    #[msg("Too early for token drop")]
    TooEarlyForTokenDrop,
    #[msg("Invalid fee value. Fee must be between 0 and 1")]
    InvalidFeeValue,
    #[msg("Couldn't get company by index")]
    CompanyByIndexError,
    #[msg("Cannot refer yourself")]
    CannotReferYourself,
    #[msg("Top donation list already contains account")]
    DuplicateInTop,
    #[msg("Illegal token vault owner")]
    IllegalTokenVaultOwner,
}
