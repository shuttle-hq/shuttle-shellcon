FROM mcr.microsoft.com/devcontainers/rust:latest

# Install Node.js 20 for frontend development
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Update Rust and install Shuttle CLI
RUN rustup update stable && \
    cargo install cargo-shuttle --locked && \
    rm -rf /usr/local/cargo/registry

ENV RUST_BACKTRACE=1
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspaces