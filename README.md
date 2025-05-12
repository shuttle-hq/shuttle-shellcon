# ShellCon Smart Aquarium System

Welcome to the emergency response team for ShellCon's Smart Aquarium System! 🦞

## The Situation

ShellCon, the world's premier crustacean convention and tech showcase, is in crisis! The revolutionary Smart Aquarium system that controls the environment for rare and valuable crustacean specimens has malfunctioned on opening day. As a software engineer from AquaTech (the company that built the system), you've been called in for emergency troubleshooting.

The system is experiencing multiple failures across its components, and critical tank conditions are deteriorating. You need to fix the issues before rare specimens are lost and the convention is ruined!

## Getting Started

### Prerequisites

- [Shuttle](https://www.shuttle.dev/) account with Pro trial
- [Rust](https://www.rust-lang.org/tools/install) (1.65 or newer)
- [Node.js](https://nodejs.org/) (16 or newer)

### Quick Start

The fastest way to get started is using VS Code with the Dev Containers extension:

1. Clone this repository
2. Open in VS Code and click "Reopen in Container" when prompted
3. Terminal will automatically install Shuttle CLI and dependencies
4. Start the local development environment:

```bash
# In terminal 1: Start the aqua-monitor service
cd services/aqua-monitor
shuttle run

# In terminal 2: Start the species-hub service
cd services/species-hub
shuttle run

# In terminal 3: Start the aqua-brain service
cd services/aqua-brain
shuttle run

# In terminal 4: Start the frontend
cd frontend
npm run dev
```

Open your browser to http://localhost:5173 (or the URL shown in the terminal)

### Manual Setup

If you're not using Dev Containers:

1. Install Shuttle CLI: `cargo install cargo-shuttle`
2. Set up Shuttle: `cargo shuttle login`
3. Install frontend dependencies: `cd frontend && npm install`

## Production Deployment

To deploy the Smart Aquarium System to production using Shuttle, follow these comprehensive steps:

### 1. Setup Prerequisites

Ensure you have the following properly configured:

- A [Shuttle](https://www.shuttle.dev/) account with Pro trial activated
- Shuttle CLI installed and logged in: `shuttle login`
- PostgreSQL database for the species-hub service (Shuttle provides this)

### 2. Database Configuration

The `species-hub` service requires a PostgreSQL database. Shuttle will automatically provision and manage this for you when you deploy.

The database schema will be automatically initialized during the first deployment. No manual migration steps are required.

### 3. Deploy Backend Services

Deploy the services in this recommended order to minimize dependency issues:

```bash
# First, deploy species-hub (database service)
cd services/species-hub
shuttle deploy
# Note the URL that Shuttle assigns to this service

# Next, deploy aqua-monitor
cd ../aqua-monitor
shuttle deploy
# Note the URL that Shuttle assigns to this service

# Finally, deploy aqua-brain (which depends on the other two)
cd ../aqua-brain
shuttle deploy
# Note the URL that Shuttle assigns to this service
```

After each deployment, Shuttle will display the unique URL for your service, e.g.:
- `https://aqua-brain-xyz123.shuttle.app`
- `https://aqua-monitor-abc456.shuttle.app`
- `https://species-hub-def789.shuttle.app`

> **Important**: If you're deploying these services for the first time, it may take 3-5 minutes for each service to fully initialize. Be patient if connections between services don't work immediately.

### 4. Configure Frontend Environment

Now that all backend services are deployed, you need to connect the frontend to them:

1. Create a `.env` file in the frontend directory based on the template:

```bash
cd frontend
cp .env.example .env
```

2. Edit the `.env` file with your actual Shuttle service URLs:

```
VITE_API_GATEWAY_URL=https://aqua-brain-xyz123.shuttle.app
VITE_MONITOR_SERVICE_URL=https://aqua-monitor-abc456.shuttle.app
VITE_SPECIES_SERVICE_URL=https://species-hub-def789.shuttle.app
```

### 5. Test Backend Connections

Before building the frontend, verify that all backend services are accessible and functioning:

```bash
# Test aqua-brain service (API Gateway)
curl https://aqua-brain-xyz123.shuttle.app/api/health

# Test aqua-monitor service
curl https://aqua-monitor-abc456.shuttle.app/api/health

# Test species-hub service
curl https://species-hub-def789.shuttle.app/api/health
```

Each service should respond with a 200 OK status.

### 6. Build and Deploy the Frontend

With the backend services confirmed working, build and deploy the frontend:

```bash
cd frontend
npm install        # Install dependencies
npm run build      # Create production build
```

Deploy the generated `dist` directory to your preferred static hosting service:

- [Netlify](https://www.netlify.com/): Drag and drop the `dist` folder or configure with GitHub
- [Vercel](https://vercel.com/): Use `vercel --prod` or connect to GitHub
- [GitHub Pages](https://pages.github.com/): Push the `dist` folder to a `gh-pages` branch
- [Cloudflare Pages](https://pages.cloudflare.com/): Upload the `dist` folder

> **Note**: Make sure your hosting provider allows you to set environment variables if you want to change the backend URLs without rebuilding.

## CORS Configuration

The backend services are already configured to accept requests from any origin during development. For production, you may want to restrict this to only your frontend domain.

If you need to customize CORS settings, look for the CORS middleware configuration in each service's `main.rs` file.

## Troubleshooting

### Backend Connection Issues

If your frontend can't connect to the backend services:

1. **Check Service Health**: Verify each service is running using the /api/health endpoint
2. **Verify Environment Variables**: Ensure your `.env` file contains the correct Shuttle URLs
3. **CORS Issues**: Check browser console for CORS errors - you may need to update the allowed origins
4. **Service Initialization**: New deployments may take a few minutes to fully initialize
5. **Database Connection**: If species-hub fails, verify the PostgreSQL database was created by Shuttle

### Challenge Detection Issues

If challenge completion is not being detected:

1. Check the logs for each service to see if events are being properly emitted
2. Verify that the `aqua-brain` service can communicate with the other services
3. Test challenge completion manually by setting environment variables (e.g., `CHALLENGE_1_SOLVED=true`)

### Common Error Messages

- **"Failed to fetch"**: Backend service is unreachable - check URLs and service status
- **"Unexpected token in JSON"**: Invalid response format - check API endpoint implementation
- **"CORS error"**: Backend service is not allowing requests from your frontend origin

```bash
# Install Shuttle CLI
curl -sSf https://docs.shuttle.dev/install.sh | sh

# Install frontend dependencies
cd frontend
npm install

# Start the services (in separate terminals)
cd services/aqua-monitor && shuttle run
cd services/species-hub && shuttle run
cd services/aqua-brain && shuttle run

# Start the frontend
cd frontend && npm run dev
```

## System Architecture

The Smart Aquarium System consists of three microservices:

- **aqua-monitor**: Environmental monitoring service
  - Collects real-time sensor data (temperature, pH, oxygen levels)
  - Manages sensor connections
  - Sends alerts when conditions are outside safe parameters

- **species-hub**: Species information service
  - Maintains database of species and their optimal conditions
  - Manages feeding schedules
  - Provides care recommendations

- **aqua-brain**: Analysis and orchestration service
  - Correlates data from other services
  - Detects patterns and anomalies
  - Provides dashboard API for frontend

## Emergency Challenges

The system is experiencing 5 critical issues that you need to fix:

1. **Environmental Monitoring System**: High latency in sensor readings
2. **Species Database**: Inefficient queries causing incomplete data
3. **Feeding System**: Poor error handling causing crashes
4. **Remote Monitoring**: Resource leakage in sensor connections
5. **Analysis Engine**: Concurrency bottleneck preventing timely analysis

Each challenge is documented in the `challenges/` directory with detailed information and hints.

## Deployment

To deploy your fixes to Shuttle:

```bash
cd services/aqua-monitor
shuttle deploy

cd services/species-hub
shuttle deploy

cd services/aqua-brain
shuttle deploy
```

## Monitoring

The system includes comprehensive instrumentation with tracing and metrics:

- Request latency for API endpoints
- Database query performance
- Resource usage (connections, memory)
- Error rates and types

These metrics are displayed in the dashboard and used to verify your fixes.

## Contributing

This project is an educational tool for learning Rust backend development with Shuttle. If you have suggestions for improvements, please open an issue or pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

Good luck, engineer! The future of ShellCon depends on your Rust expertise! 🦀
