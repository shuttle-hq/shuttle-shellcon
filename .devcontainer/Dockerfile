FROM mcr.microsoft.com/devcontainers/rust:latest

# Install system dependencies without apt-get to avoid GPG issues
# Use the base image which already has most tools we need
RUN cargo install cargo-shuttle --locked

ENV RUST_BACKTRACE=1
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspaces