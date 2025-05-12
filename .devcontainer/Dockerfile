FROM mcr.microsoft.com/devcontainers/rust:latest

# Install additional packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install postgresql-client curl wget lsof htop jq netcat-openbsd \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Install Shuttle CLI
RUN curl -sSf https://docs.shuttle.dev/install.sh | sh

# Set up environment variables
ENV RUST_BACKTRACE=1
ENV DATABASE_URL=postgres://postgres:postgres@localhost:5432/species_hub

# Create helper scripts for managing services
RUN mkdir -p /usr/local/bin/aquarium-helpers

WORKDIR /usr/local/bin/aquarium-helpers

# Create helper script for running all services
RUN echo '#!/bin/bash' > /usr/local/bin/run-aquarium.sh && \
    echo 'echo "Starting PostgreSQL..."' >> /usr/local/bin/run-aquarium.sh && \
    echo 'sudo service postgresql start' >> /usr/local/bin/run-aquarium.sh && \
    echo '' >> /usr/local/bin/run-aquarium.sh && \
    echo 'echo "Creating database if it does not exist..."' >> /usr/local/bin/run-aquarium.sh && \
    echo 'sudo -u postgres psql -c "CREATE DATABASE species_hub;" || true' >> /usr/local/bin/run-aquarium.sh && \
    echo '' >> /usr/local/bin/run-aquarium.sh && \
    echo 'echo "Starting all services in separate terminals (use Ctrl+C to exit all)..."' >> /usr/local/bin/run-aquarium.sh && \
    echo 'tmux new-session -d -s aquarium "cd /workspaces/shellcon/services/species-hub && shuttle run; read"' >> /usr/local/bin/run-aquarium.sh && \
    echo 'tmux split-window -h "cd /workspaces/shellcon/services/aqua-monitor && shuttle run; read"' >> /usr/local/bin/run-aquarium.sh && \
    echo 'tmux split-window -v "cd /workspaces/shellcon/services/aqua-brain && shuttle run; read"' >> /usr/local/bin/run-aquarium.sh && \
    echo 'tmux split-window -h "cd /workspaces/shellcon/frontend && npm run dev; read"' >> /usr/local/bin/run-aquarium.sh && \
    echo 'tmux -2 attach-session -d' >> /usr/local/bin/run-aquarium.sh

# Make script executable
RUN chmod +x /usr/local/bin/run-aquarium.sh

# Link script to path for easy access
RUN ln -s /usr/local/bin/aquarium-helpers/run-aquarium.sh /usr/local/bin/run-aquarium

# Install tmux for running multiple services at once
RUN apt-get update && apt-get install -y tmux && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /workspaces
