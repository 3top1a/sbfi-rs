#!/bin/bash

# Install nightly toolchain
# rustup toolchain install nightly-x86_64-unknown-linux-gnu
# rustup component add rust-src --toolchain nightly

RUSTFLAGS="-C target-cpu=native -C link-args=-nostartfiles -C target-feature=+crt-static -C relocation-model=static -C link-args=-Wl,--gc-sections,-n,-N,--no-dynamic-linker,--no-pie,-build-id=none" cargo +nightly b --release --target x86_64-unknown-linux-gnu;

# Processing
# Copy to root
cp target/x86_64-unknown-linux-gnu/release/sbfi sbfi;

# Remove unnecesary sectors
objcopy -R .shstrtab -R .comment sbfi sbfi.tmp;
mv sbfi.tmp sbfi;

# sstrip from https://github.com/BR903/ELFkickers, highly recommend that
sstrip -z sbfi

# Remove ud2 instructions
# TODO
# https://clang.llvm.org/docs/AddressSanitizer.html

# Put into dropper
# Credit https://in4k.github.io/wiki/linux
lzip -9 -c sbfi | cat sh.template - > run.sh
chmod +x run.sh

echo;
echo "Final binary size:";
/bin/ls -l ./run.sh | awk '{print $5}';
file ./run.sh;

#./sbfi
# Should print �%
echo "-." | ./run.sh;
echo;
echo "+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+." | ./run.sh;
