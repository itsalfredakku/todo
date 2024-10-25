#!/bin/bash

# Install SurrealDB CLI
curl -sSf https://install.surrealdb.com | sh

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


# Install Cloudflared
curl -L --output cloudflared.deb https://github.com/cloudflare/cloudflared/releases/latest/download/cloudflared-linux-amd64.deb
# Install the package
sudo dpkg -i cloudflared.deb
