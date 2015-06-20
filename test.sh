#!/bin/zsh

if diff -q test_results/header.pass.diff <(diff <(cargo run -- -h test_programs/be/a.out) <(readelf -h test_programs/be/a.out)) 2>&1 > /dev/null; then
    echo "Tests pass";
else
    echo "Tests fail";
fi;
