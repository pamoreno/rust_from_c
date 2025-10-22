#!/bin/bash

cd some_lib
cargo test --release
cargo build --release
cd -
gcc main.c -L./some_lib/target/release -I. -lsome_lib -o app
