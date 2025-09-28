use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Offered is should be greater than 0")]
    InvalidOfferedAmount,

    #[msg("Invalid Mint Account")]
    InvalidMintAccount,

    #[msg("Wanted Should be greater than 0")]
    InvalidWantedAmount,

    #[msg("Amount is not equal to the wanted amount")]
    WantedAmountMismatch,

    #[msg("Mint Account Mismatch")]
    MintAccountMisMatch,

    #[msg("Error Closing Vault")]
    FailedVaultClosure,

    #[msg("Error transferring token from taker to maker")]
    InsufficientTakerBalance,

    #[msg("Failed to withdraw token from vault")]
    FailedVaultWithdrawal,
}
