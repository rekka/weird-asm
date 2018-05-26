#!/bin/sh

RUSTFLAGS="-C target-cpu=native" cargo build --release
objdump -d target/release/weird-asm > objdump.txt 

RUSTFLAGS="-C target-cpu=native --emit=asm" cargo build --release  
objdump -d target/release/weird-asm > objdump-emit-asm.txt 

