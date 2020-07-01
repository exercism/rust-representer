#!/usr/bin/env sh

rustfmt ${2}src/lib.rs
/opt/representer/bin/representer --slug $1 --path $2
rustfmt ${2}representation.rs
cat ${2}representation.rs > ${2}representation.txt