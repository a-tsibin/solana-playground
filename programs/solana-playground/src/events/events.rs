use anchor_lang::prelude::*;

#[event]
pub struct CompanyStartedEvent {
    pub initiator: Pubkey,
    pub company_id: u16,
}

#[event]
pub struct WithdrawEvent {
    pub company_id: u16,
    pub amount: u64,
}

#[event]
pub struct WithdrawFeesEvent {}

#[event]
pub struct DonationEvent {
    pub company_id: u16,
    pub amount: u64,
}

#[event]
pub struct CloseCompanyEvent {
    pub company_id: u16,
}

#[event]
pub struct TokenDropEvent {
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct TokenTransferEvent {
    pub company_id: u16,
    pub amount: u64,
}

#[event]
pub struct CompanyClosedByTokensEvent {
    pub company_id: u16,
}
