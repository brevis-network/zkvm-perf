// This code is borrowed from RISC Zero's benchmarks.
//
// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

#[cfg(feature = "risc0")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1")]
sp1_zkvm::entrypoint!(main);

#[cfg(target_os = "zkvm")]
use core::arch::asm;

fn main() {
    #[cfg(feature = "sp1")]
    let data: Vec<u8> = sp1_zkvm::io::read();
    #[cfg(feature = "risc0")]
    let data: Vec<u8> = risc0_zkvm::guest::env::read();

    #[cfg(feature = "sp1")]
    {

    }

    #[cfg(feature = "risc0")]
    {
        use risc0_zkvm::{guest::env, sha, sha::Sha256};
        let data: Vec<u8> = env::read();
        let hash = sha::Impl::hash_bytes(&data);
        env::commit(hash)
    }
}