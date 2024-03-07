#!/bin/sh
#
echo "Building client"
cargo build --release

program_address=$1
source_address=$2
signer=$3

./target/release/client copy-transaction $program_address $source_address --signer $signer -r htt -w ws:
