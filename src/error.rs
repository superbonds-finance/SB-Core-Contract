use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum SB_Error {

    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("SignatureMissing")]
    SignatureMissing,

    #[error("AlreadyInUse")]
    AlreadyInUse,

    #[error("IncorrectTokenProgramId")]
    IncorrectTokenProgramId,

    #[error("ExpectedAccount")]
    ExpectedAccount,

    #[error("ExpectedMint")]
    ExpectedMint,

    #[error("ABOVE_LIQ_AVALABILITY")]
    ABOVE_LIQ_AVALABILITY,

    #[error("Invalid_SUPERB_FEE")]
    Invalid_SUPERB_FEE,

    #[error("Invalid_NFT_AMOUNT")]
    Invalid_NFT_AMOUNT,

    #[error("Invalid_NFT_MINT")]
    Invalid_NFT_MINT,

    #[error("Invalid_NFT_MINT_NOT_MATCH")]
    Invalid_NFT_MINT_NOT_MATCH,

    #[error("Invalid_NFT_OWNER")]
    Invalid_NFT_OWNER,

    #[error("NFT_NOT_MATCH")]
    NFT_NOT_MATCH,

    #[error("LP_PRICE_ZERO")]
    LP_PRICE_ZERO,

    #[error("ZERO_INPUT")]
    ZERO_INPUT,

    #[error("INVALID_REDEEM_TIME")]
    INVALID_REDEEM_TIME,

    #[error("INVALID_TRADER_DATA_OWNER")]
    INVALID_TRADER_DATA_OWNER,

    #[error("Invalid_Staked_SB_TokenBalance")]
    Invalid_Staked_SB_TokenBalance,

    #[error("Pool_inactive")]
    Pool_inactive,

    #[error("MathOverflow")]
    MathOverflow,

    #[error("OverFlow")]
    OverFlow,

}

impl From<SB_Error> for ProgramError {
    fn from(e: SB_Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}
