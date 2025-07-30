# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ShellCon is an educational full-stack smart aquarium management system designed for learning Rust and Shuttle Cloud deployment. The project includes a React frontend dashboard and three Rust microservices that form a complete aquarium monitoring ecosystem.

## Architecture

The system follows a microservices architecture with:

**Backend Services** (located in `shellcon-backend/services/`):
- `aqua-monitor` (port 8000): Environmental sensor readings and tank monitoring
- `species-hub` (port 8001): Species database and feeding schedules  
- `aqua-brain` (port 8002): Analytics engine for tank health analysis

**Frontend** (located in `shellcon-frontend/`):
- React + Vite dashboard with TypeScript
- Uses shadcn/ui components and Tailwind CSS
- Connects to all three backend services via REST APIs

## Development Commands

### Backend Services
Each Rust service uses Shuttle for local development and deployment:

```bash
# Run individual services locally (from service directory)
cd shellcon-backend/services/aqua-monitor && shuttle run --port 8000
cd shellcon-backend/services/species-hub && shuttle run --port 8001  
cd shellcon-backend/services/aqua-brain && shuttle run --port 8002

# Deploy to Shuttle Cloud (from service directory)
shuttle deploy

# View service logs
shuttle logs --latest
```

### Frontend
```bash
cd shellcon-frontend

# Install dependencies (first time)
npm install

# Development servers for different environments
npm run dev:localhost    # Points to localhost:8000-8002
npm run dev:prod        # Points to deployed Shuttle services

# Build and lint
npm run build
npm run lint
```

## DevContainer Setup

A DevContainer configuration is available for Docker-based development environments. The setup includes:

- Pre-configured Rust development environment with Microsoft's official container
- Node.js 20 for frontend development  
- Docker-in-Docker support via DevContainer features
- VS Code extensions for Rust development
- Automatic Shuttle CLI installation via postCreateCommand

### Docker-in-Docker Database Connectivity

**Important**: The DevContainer includes a fix for Shuttle's database connectivity in Docker-in-Docker scenarios. When Shuttle runs locally with a database, it provisions a PostgreSQL container with a connection string using `localhost`. However, inside a dev container, `localhost` points to the container itself, not the host where the database container runs.

The DevContainer configuration includes:
```json
"runArgs": [
  "--add-host=host.docker.internal:host-gateway"
]
```

This maps the container's default bridge gateway to `host.docker.internal`, allowing the Shuttle services to reach the host-provisioned database containers. This resolves the connectivity issue without requiring manual database provisioning.

### Using the DevContainer

1. Open the project in VS Code
2. When prompted, select "Reopen in Container" or use Command Palette → "Dev Containers: Reopen in Container" 
3. **First time setup** (30 seconds): Run `./.devcontainer/setup-node.sh` to install Node.js 20
4. Run `shuttle login` to authenticate
5. Follow normal development workflow with `shuttle run` commands

**Fast startup**: The DevContainer now starts in ~15 seconds with minimal extensions, then you can install Node.js only when needed for frontend development.

## Challenge System

The project includes 4 performance optimization challenges embedded in the Rust services:

1. **Challenge 1** (`aqua-monitor/src/challenges.rs::get_tank_readings`): Async I/O optimization
2. **Challenge 2** (`species-hub/src/challenges.rs::get_species`): SQL query optimization  
3. **Challenge 3** (`aqua-brain/src/challenges.rs::get_analysis_result`): String allocation optimization
4. **Challenge 4** (`aqua-monitor/src/challenges.rs::get_sensor_status`): HTTP client resource pooling

Each challenge has validation endpoints that check implementation patterns in the source code. Solutions must be implemented within the marked `// ⚠️ CHALLENGE` code blocks.

## Environment Configuration

Frontend supports multiple environment configurations via `.env` files:

- `.env.localhost` - Used with `npm run dev:localhost` for local backend services
- `.env.prod` - Used with `npm run dev:prod` for deployed Shuttle services

Required environment variables:
```
VITE_AQUA_MONITOR_URL=http://localhost:8000 (or deployed URL)
VITE_SPECIES_HUB_URL=http://localhost:8001 (or deployed URL)  
VITE_AQUA_BRAIN_URL=http://localhost:8002 (or deployed URL)
VITE_API_BASE_URL=/api
```

## Key Technical Details

- **Database**: Each service uses PostgreSQL via Shuttle's shared database feature
- **HTTP Framework**: All services use Axum with tower-http for CORS and tracing
- **Async Runtime**: Tokio for all async operations
- **Tracing**: Structured logging with tracing crate and correlation IDs
- **Error Handling**: Custom error types implementing IntoResponse for consistent API responses
- **Frontend State**: React Query for API state management
- **UI Components**: shadcn/ui component library with Radix UI primitives

## Development Workflow

1. Start all three backend services in separate terminals
2. Start frontend with appropriate environment (localhost/prod)
3. Use the dashboard to test all API endpoints via the System Control Panel
4. Implement challenge solutions within marked code blocks
5. Use the Validate buttons in the UI to verify implementations
6. Deploy services to Shuttle Cloud when ready

## Important Notes

- Services must be restarted after code changes to challenges
- The validation system checks source code patterns, not just runtime behavior
- Port conflicts can be resolved with `lsof -ti :<port> | xargs kill -9`
- All services require `shuttle login` authentication before deployment
- Challenge solutions must maintain the same API contracts while improving performance