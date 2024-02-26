#!/bin/bash
# set -e

cd $(dirname "$0")
mkdir -p logs
DATE=$(date "+%Y_%m_%d")

cargo run --release 2>&1 | tee -a logs/$DATE.txt
