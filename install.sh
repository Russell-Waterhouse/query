#!/bin/bash


cargo build --release
cp ./target/release/query ~/bin
