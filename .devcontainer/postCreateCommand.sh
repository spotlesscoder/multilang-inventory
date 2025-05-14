#!/bin/bash

sudo apt update
sudo apt upgrade -y
sudo apt install -y maven
rustup component add --toolchain stable-x86_64-unknown-linux-gnu rustfmt clippy
npm ci
