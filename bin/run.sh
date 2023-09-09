#!/usr/bin/env sh

rustfmt ${2}src/lib.rs
/opt/representer/bin/rust-representer --slug $1 --input-path $2 --output-path $3
rustfmt ${3}representation.rs
cat ${3}representation.rs > ${3}representation.txt