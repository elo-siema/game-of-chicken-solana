use anchor_lang::prelude::ProgramError;
// inside error.rs
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum ChickenError {
    #[error("Start date is in the past")]
    StartDateInThePast,
    #[error("End date is in the past")]
    EndDateInThePast,
    #[error("Start date is after the end date!")]
    StartDateAfterEnd,
}

impl From<ChickenError> for ProgramError {
    fn from(e: ChickenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}