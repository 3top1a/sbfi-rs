#!/bin/bash

# Install nightly toolchain
# rustup toolchain install nightly-x86_64-unknown-linux-gnu
# rustup component add rust-src --toolchain nightly

RUSTFLAGS="-Ctarget-cpu=native -Clink-args=-nostartfiles -Ctarget-feature=+crt-static -C relocation-model=static -Clink-args=-Wl,-n,-N,--no-dynamic-linker,--no-pie,-build-id=none " cargo +nightly b --release --target x86_64-unknown-linux-gnu;

# Processing
# Copy to root
cp target/x86_64-unknown-linux-gnu/release/sbfi sbfi;

# Remove unnecesary sectors
objcopy -R .shstrtab -R .comment sbfi sbfi.tmp;
mv sbfi.tmp sbfi;

# sstrip from https://github.com/BR903/ELFkickers, highly recommend that
#sstrip -z sbfi

# Remove ud2 instructions
# TODO
# https://clang.llvm.org/docs/AddressSanitizer.html

echo;
echo "Final binary size:";
/bin/ls -l sbfi | awk '{print $5}';
file sbfi;

#./sbfi
# Should print �%
echo "-." | ./sbfi;
echo "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+." | ./sbfi;
