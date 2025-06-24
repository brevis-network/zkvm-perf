#!/bin/bash
git clone https://github.com/risc0/risc0.git
cd riscv0
git switch --detach v3.0.0-rc.1
cargo uninstall rzup
cargo run --bin rzup install