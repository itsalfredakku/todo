# Start with the Rust dev container image as the base
FROM mcr.microsoft.com/devcontainers/rust:latest

# Install Node.js (replace with desired Node.js version)
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# Verify installations
RUN rustc --version && node --version && npm --version
