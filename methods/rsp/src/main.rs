#![no_main]
risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env;

use rsp_client_executor::{
    executor::{EthClientExecutor, DESERIALZE_INPUTS},
    io::EthClientExecutorInput,
};
use std::sync::Arc;

pub fn main() {
    // Read the input.
    println!("cycle-tracker-report-start: {}", DESERIALZE_INPUTS);
    let input: Vec<u8> = env::read();
    let input = bincode::deserialize::<EthClientExecutorInput>(&input).unwrap();
    println!("cycle-tracker-report-end: {}", DESERIALZE_INPUTS);

    // Execute the block.
    let executor = EthClientExecutor::eth(
        Arc::new((&input.genesis).try_into().unwrap()),
        input.custom_beneficiary,
    );
    let header = executor.execute(input).expect("failed to execute client");
    let block_hash = header.hash_slow();

    // Commit the block hash.
    env::commit(&block_hash);
}
