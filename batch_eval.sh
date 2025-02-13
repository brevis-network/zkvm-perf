#!/bin/bash

commands=(
  "./eval.sh fibonacci300kn sp1 poseidon 21 benchmark  > logs/fibo-300kn.log"
  "./eval.sh rsp17106222 sp1 poseidon 21 benchmark 17106222 > logs/reth-171.log"
  "./eval.sh tendermint sp1 poseidon 21 benchmark > logs/tendermint.log"
  "./eval.sh rsp20528709 sp1 poseidon 21 benchmark 20528709 > logs/reth-20528709.log"
)

for cmd in "${commands[@]}"; do
  echo "Executing: $cmd"
  eval $cmd
done
