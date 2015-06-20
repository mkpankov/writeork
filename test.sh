#!/bin/zsh
set -e

diff test_results/header.pass.diff <(diff <(cargo run -- -h test_programs/be/a.out) <(readelf -h test_programs/be/a.out))
