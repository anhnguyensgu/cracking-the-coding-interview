use borsh::BorshDeserialize as _;
use borsh_derive::{BorshDeserialize, BorshSerialize};
// use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct UpdateArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(UpdateArgs),
    Decrement(UpdateArgs),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        solana_program::msg!("split {:?}", rest);

        Ok(match variant {
            0 => Self::Increment(UpdateArgs::try_from_slice(rest)?),
            1 => Self::Decrement(UpdateArgs::try_from_slice(rest)?),
            2 => Self::Update(UpdateArgs::try_from_slice(rest)?),
            3 => Self::Reset,
            _ => Err(ProgramError::InvalidInstructionData)?,
        })
    }
}
