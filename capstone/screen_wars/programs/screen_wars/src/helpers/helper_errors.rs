use anchor_lang::prelude::*;

#[error_code]
pub enum TransferErrors {
    #[msg("PDA must remain rent-exempt after transfer")]
    PDAInsufficientRent,
}

#[error_code]
pub enum ArthemeticErrors {
    #[msg("Operation caused buffer overflows")]
    IntegerOverflow,

    #[msg("Operation caused buffer underflows")]
    IntegerUnderflow,
}
