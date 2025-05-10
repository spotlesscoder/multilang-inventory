#!/bin/bash

sudo apt update
sudo apt upgrade -y
sudo apt install -y maven
rustup component add clippy
rustup component add rustfmt
npm ci
