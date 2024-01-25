use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct IncValue {
    pub value: u32,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct DecValue {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(IncValue),
    Decrement(DecValue),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(IncValue::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(DecValue::try_from_slice(rest).unwrap()),
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
