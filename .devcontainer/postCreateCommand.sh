#!/bin/bash

sudo apt update
sudo apt upgrade -y
rustup component add clippy
curl -fsSL https://get.pnpm.io/install.sh | sh -
source /home/vscode/.bashrc
