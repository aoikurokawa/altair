#!/bin/sh
#
echo "Get PDA"
cargo build --release

program_address=$1
source_address=$2

./target/release/client copy-pda $program_address $source_address
