FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && \
    apt-get install -y docker.io curl wget htop && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-shuttle

ENV RUST_BACKTRACE=1
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspaces
