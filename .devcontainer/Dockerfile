FROM mcr.microsoft.com/devcontainers/rust:latest

# Install Node.js 20 for frontend development
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install latest Shuttle CLI
RUN cargo install cargo-shuttle --locked && \
    cargo cache --autoclean

ENV RUST_BACKTRACE=1
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspaces