#!/bin/bash

cargo build --bins

for day in $(seq -w 1 16); do
  echo "Running day_$day..."
  cargo run --release --quiet --bin day_$day
done
