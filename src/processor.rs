// Instructor processor

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo},
    log::{sol_log_compute_units, sol_log_params},
    msg,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Greetings {
    pub counter: u32,
}

/*
    Implement program entrypoint. The entrypoint always calls 
    the instructor processor function (i.e. 'helloworld')
    with these 3 params: program_id, accounts, instruction_data.
    You must declare ALL 3 params them even if unused.
    Add an _ in front of the variable name, if unused.
 */ 
pub fn process_instruct(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {


    let mut greet = Greetings { counter: 0u32 };
    greet.counter += 1;

    /*
        LOG EVENTS
     */
    // log a string
    msg!("Print a static string!");
    // log a formatted string, beware: expensive!
    msg!("Greeted {} time(s)!", greet.counter);
    // log a public key
    program_id.log();
    // log all the program's input parameters
    sol_log_params(accounts, instruction_data);
    // log the number of compute units remaining to consume
    sol_log_compute_units();


    Ok(())
}