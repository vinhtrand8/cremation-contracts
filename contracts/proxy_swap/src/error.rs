use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Locked")]
    Locked {},
    #[error("Already unlocked")]
    AlreadyUnlocked {},
    #[error("Invalid Reply Message")]
    InvalidReplyMsg {},
    #[error("Expect Only One Coin")]
    ExpectOnlyOneCoin {},
    #[error("Zero Amount")]
    ZeroAmount {},
    #[error("Invalid Tax Rate")]
    InvalidTaxRate {},
    #[error("Invalid Ask Asset")]
    InvalidAskAsset {},
}
