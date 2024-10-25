#!/bin/bash

# Install NVM and Node.js
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
export NVM_DIR="$HOME/.nvm"
# Load NVM
[ -s "$NVM_DIR/nvm.sh" ] && . "$NVM_DIR/nvm.sh"
# Install and set Node.js version
nvm install 20
nvm use 20
nvm alias default 20

# Install Rust CLI tool (sqlx-cli)
cargo install sqlx-cli
