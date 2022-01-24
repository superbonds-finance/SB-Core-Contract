use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum SB_Error {

    #[error("SignatureMissing")]
    SignatureMissing,

    #[error("ExpectedAccount")]
    ExpectedAccount,

    #[error("ExpectedMint")]
    ExpectedMint,

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
