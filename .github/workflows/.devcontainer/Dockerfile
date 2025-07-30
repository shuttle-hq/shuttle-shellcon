FROM mcr.microsoft.com/devcontainers/rust:latest

# Install Shuttle CLI - the base image already has most tools we need
RUN cargo install cargo-shuttle --locked

ENV RUST_BACKTRACE=1
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspaces
