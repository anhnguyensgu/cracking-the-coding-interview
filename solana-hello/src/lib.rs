pub mod instruction;

use crate::instruction::CounterInstructions;
use borsh::{BorshDeserialize as _, BorshSerialize as _};
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstructions::unpack(instruction_data)?;
    let accounts_iter = &mut accounts.iter();
    let account = solana_program::account_info::next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;
    // log a message to the blockchain
    match instruction {
        CounterInstructions::Increment(args) => {
            counter_account.counter += args.value;
        }
        CounterInstructions::Decrement(args) => {
            counter_account.counter -= args.value;
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    // gracefully exit the program
    Ok(())
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

#[cfg(test)]
mod test {
    use std::mem;

    use borsh::BorshDeserialize;
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey, stake_history::Epoch};

    use crate::{process_instruction, CounterAccount};

    #[test]
    fn should_work() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let accounts = vec![account];

        // let increment_instruction_data: Vec<u8> = vec![0];
        // let decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![];
        let update_value = 32u32;
        update_instruction_data.push(2);
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());
        // let argsObject = UpdateArgs{value: 5};
        // let reset_instruction_data: Vec<u8> = vec![3];

        // process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();
        // assert_eq!(
        //     CounterAccount::try_from_slice(&accounts[0].data.borrow())
        //         .unwrap()
        //         .counter,
        //     1
        // );
        //
        // process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();
        // assert_eq!(
        //     CounterAccount::try_from_slice(&accounts[0].data.borrow())
        //         .unwrap()
        //         .counter,
        //     0
        // );

        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
    }
}
