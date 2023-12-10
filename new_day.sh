#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "please provide one arg"
    exit 1
fi

cargo new "$1"
cp ./template.rs "$1/src/main.rs"
touch "$1/example.txt"
touch "$1/real.txt"

