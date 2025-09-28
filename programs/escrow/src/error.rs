use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Offered is should be greater than 0")]
    InvalidOfferedAmount,

    #[msg("Wanted Should be greater than 0")]
    InvalidWantedAmount,
}
