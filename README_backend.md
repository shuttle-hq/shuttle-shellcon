# 🦀 ShellCon Smart Aquarium System 🦀

Welcome to the ShellCon Smart Aquarium System! This interactive project will introduce you to building high-performance microservices with Rust and Shuttle Cloud.

## 🌊 The ShellCon Scenario

Imagine you've just joined the emergency technical response team for ShellCon, the world's premier convention for Rustaceans and crustaceans alike! This year's main attraction is a revolutionary Smart Aquarium system built with Rust and deployed on Shuttle.

**The problem?** Just hours before the convention opens, several performance issues have been detected in the backend services. As the newly recruited Rustacean engineer, you've been called in to optimize these systems before the doors open to the public.

The convention organizers are in a pinch—quite literally, as the convention's mascot, a giant Coconut Crab named Ferris, is anxiously clicking his claws at the mounting technical issues!

## 🏗️ System Architecture

The Smart Aquarium System consists of three backend microservices and a separate frontend application:

### Backend Services
- **aqua-monitor**: Collects real-time environmental data from tank sensors
- **species-hub**: Manages the species database and feeding requirements
- **aqua-brain**: Performs analysis and coordinates system responses

### Frontend Application
The frontend application is available in a separate repository at [shuttle-shellcon-frontend](https://github.com/shuttle-hq/shuttle-shellcon-frontend). The UI provides:
- A description of the challenges to solve (They can also be found in this repository under the `challenges` folder)
- Interactive challenge validation
- Detailed lecture materials for each challenge
- Visual feedback on your solutions
- Real-time monitoring of your aquarium system

To get started with the frontend:
1. Clone the frontend repository (Not now, you will do it later as indicated in step 4 below)
2. Follow the setup instructions in its README
3. Configure it to point to your deployed services

## 🚀 Getting Started

Shuttle is a platform that makes deploying Rust applications simple. For best productivity, you'll iterate locally with Shuttle, then deploy to the cloud for final validation.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or newer)
- [Docker](https://www.docker.com/products/docker-desktop/) (for local database provisioning and running services locally)
  - **Important**: In Docker Desktop settings, enable "Allow the default Docker socket to be used" option
- [Shuttle CLI](https://docs.shuttle.dev/getting-started/installation) (latest version)

### 1. Install Shuttle CLI

If you don't have a Shuttle account, please create one [here](https://console.shuttle.dev/login).

```bash
# Install the Shuttle CLI
curl -sSfL https://www.shuttle.dev/install | bash
```

### 2. Login to Shuttle

Login right after installing the CLI to avoid interruptions later:

```bash
shuttle login
```

### 3. Run Services Locally (Recommended for Fast Iteration)

Open three terminal windows/tabs and run each service on its designated port:

```bash
# Terminal 1
cd services/aqua-monitor
shuttle run --port 8000

# Terminal 2
cd services/species-hub
shuttle run --port 8001

# Terminal 3
cd services/aqua-brain
shuttle run --port 8002
```

- You can directly run `curl` commands to test the services by using the ports:
  - aqua-monitor: http://localhost:8000
  - species-hub: http://localhost:8001
  - aqua-brain: http://localhost:8002

For example, validate the aqua-monitor service by running:

```bash
# Test aqua-monitor service health
curl http://localhost:8000/api/health
```

Iterate and solve the challenges by editing code and re-running the services as needed.

**If you get an error about a port being in use, free it with:**
```sh
lsof -ti :<port> | xargs kill -9
```

### 4. Validate Locally

Clone the frontend repository:
```bash
git clone https://github.com/shuttle-hq/shuttle-shellcon-frontend.git
```
Then follow the setup instructions in the frontend repository's README.

- Make your code changes and run the affected service locally by killing the process and restarting the service with `shuttle run --port <port>`.
- Start the frontend with `npm run dev:localhost` (see frontend repo for setup).
- Use the frontend UI to validate your solution by clicking the "Validate your solution" button under the challenge description. This is the required method for validation.
- Optionally, use Thunder Client, curl, or Postman to test API endpoints for debugging.
- Ensure all challenges pass locally before moving to cloud deployment.

### 5. Deploy to Shuttle Cloud (Final Validation)

When your solution passes locally, deploy each service to Shuttle Cloud. Repeat these steps for all three services:

**For each service:**
1. Go to the service directory from the repository root:
   - For aqua-monitor:
     ```bash
     cd services/aqua-monitor
     ```
   - For species-hub:
     ```bash
     cd services/species-hub
     ```
   - For aqua-brain:
     ```bash
     cd services/aqua-brain
     ```
2. Verify that the `shuttle.toml` file is present in the directory. This file is required for deployment.
3. Deploy the service:
   ```bash
   shuttle deploy
   ```
4. When prompted:
   - Select `[CREATE NEW]`.
   - For the project name, use the name of the service you are currently deploying (the name of your current folder). For example, enter `aqua-monitor` if you are in the `aqua-monitor` directory.
5. After deploying, Shuttle will provide a unique URL for the service (e.g., `https://aqua-monitor-xxxx.shuttleapp.app`).

Repeat these steps for all three services.

**Update your frontend configuration:**
- Open your frontend project's `.env.prod` file.
- Copy the unique URLs for all three services (aqua-monitor, species-hub, aqua-brain) into the corresponding variables in `.env.prod`.
- Your `.env.prod` file should look like this (replace with your actual URLs):

```env
VITE_AQUA_MONITOR_URL=https://aqua-monitor-xxxx.shuttleapp.app
VITE_SPECIES_HUB_URL=https://species-hub-xxxx.shuttleapp.app
VITE_AQUA_BRAIN_URL=https://aqua-brain-xxxx.shuttleapp.app
```

- Restart the frontend with `npm run dev:prod` to connect to your cloud services.
- Use the frontend UI to validate your solution against the cloud endpoints by clicking the "Validate your solution" button.

#### Finding Your Cloud Endpoints

After deploying, Shuttle will provide a unique URL for each service. Example output:

```
Created project 'aqua-monitor' with id proj_xxxxxx
Linking to project 'aqua-monitor' with id proj_xxxxxx
Packing files...
Uploading code...
Creating deployment...
Deployment depl_xxxxxx - running
https://aqua-monitor-xxxx.shuttle.app

```

Record the URLs for:
- aqua-monitor (e.g., https://aqua-monitor-xxxx.shuttleapp.app)
- species-hub (e.g., https://species-hub-xxxx.shuttleapp.app)
- aqua-brain (e.g., https://aqua-brain-xxxx.shuttleapp.app)

**Local vs Cloud:**
- Local: Use ports (8000, 8001, 8002) with `localhost` and run frontend with `npm run dev:localhost`.
- Cloud: Use the `.shuttleapp.app` URLs and run frontend with `npm run dev:prod`.

### 6. Final Challenge Validation

Verify that all services are running and accessible:

```bash
# Test aqua-brain service
curl https://aqua-brain-xyz123.shuttle.app/api/health

# Test aqua-monitor service
curl https://aqua-monitor-abc456.shuttle.app/api/health

# Test species-hub service
curl https://species-hub-def789.shuttle.app/api/health
```

Each service should respond with a 200 OK status.

- Re-validate your solution against the cloud endpoints using the frontend UI button "Validate your solution".

## 🕹️ The Optimization Challenges

Your mission is to solve five performance challenges across the microservices. Each challenge focuses on a different aspect of backend optimization in Rust.

### Challenge 1: The Sluggish Sensor (Async I/O)
- **Service**: aqua-monitor
- **File**: src/challenges.rs
- **Function**: get_tank_readings
- **Problem**: The environmental monitoring system is experiencing severe delays due to inefficient file I/O operations.

### Challenge 2: The Query Conundrum (Database Optimization)
- **Service**: species-hub
- **File**: src/challenges.rs
- **Function**: get_species
- **Problem**: The species database is responding slowly to searches due to inefficient queries.

### Challenge 3: The Memory Miser (String Optimization)
- **Service**: aqua-brain
- **File**: src/challenges.rs
- **Function**: get_analysis_result
- **Problem**: The analysis engine is consuming excessive memory due to inefficient string handling.

### Challenge 4: The Leaky Connection (Resource Management)
- **Service**: aqua-monitor
- **File**: src/challenges.rs
- **Function**: get_sensor_status
- **Problem**: The sensor status API is creating a new HTTP client for every request, causing resource leaks.

## 🧰 How to Solve a Challenge

Follow this workflow to solve each challenge:

### 1. Understand the Problem

Examine the challenge description and the problematic code:

```bash
# View the source code for the challenge
cat services/aqua-monitor/src/challenges.rs
```
Look for the challenge tag (e.g., `// ⚠️ CHALLENGE #1: ASYNC I/O ⚠️`).

### 2. Implement Your Solution

Edit the code to fix the performance issue. You can:
- Read the challenge lecture in the UI for detailed explanations
- Click the "Show Hint" button in the UI if you're stuck
- View the solution guide for step-by-step instructions
- Check the code comments for additional hints

### 3. Verify Locally Before Deploying

```bash
# Format and check your code
cd services/aqua-monitor
cargo fmt
cargo check

# Run the service locally to verify your changes compile and run
shuttle run --port 8000
```

Make sure your frontend is running with `npm run dev:localhost` to connect to your local services.

## 💡 Challenge Tips

### Challenge 1: Async I/O
- Look for blocking I/O operations that should be async
- Consider using `tokio::fs` instead of standard `std::fs`

### Challenge 2: Database Queries
- Examine the SQL query for inefficient patterns
- Consider adding indexes or using case-insensitive search

### Challenge 3: String Optimization
- Look for excessive String allocations
- Consider using string references (&str) where appropriate

### Challenge 4: Resource Management
- Identify resources being created for each request
- Use static instances for expensive resources

## 🔧 Troubleshooting

### Deployment Issues

If your deployment fails:

```bash
# Check the deployment logs
shuttle logs --latest
```

Common issues include:
- Compilation errors
- Missing dependencies
- Configuration problems

### Validation Issues

If your solution isn't being validated correctly:

1. **Check Implementation**: Ensure your solution matches the expected pattern
2. **Verify Deployment**: Make sure your changes were properly deployed
3. **Examine Logs**: Check the service logs for validation errors

## 🏗️ System Architecture Details

The Smart Aquarium System follows a microservices architecture where each service has a specific responsibility. Importantly, **services do not communicate directly with each other** - the frontend is responsible for coordinating data between services.

### aqua-monitor

- **Purpose**: Real-time environmental monitoring service
- **Key Features**:
  - Collects sensor data (temperature, pH, oxygen, salinity)
  - Manages sensor connections and status
  - Provides historical readings and alerts
- **Tech Stack**: Rust, Axum, SQLx, PostgreSQL
- **Challenges**: Async I/O optimization, resource management

### species-hub

- **Purpose**: Species information and feeding management
- **Key Features**:
  - Maintains species database with environmental requirements
  - Manages feeding schedules and nutritional data
  - Provides species compatibility information
- **Tech Stack**: Rust, Axum, SQLx, PostgreSQL
- **Challenges**: Database query optimization

### aqua-brain

- **Purpose**: Analysis and system coordination
- **Key Features**:
  - Analyzes tank conditions and species health
  - Detects patterns and anomalies
  - Coordinates system-wide responses
- **Tech Stack**: Rust, Axum, reqwest
- **Challenges**: Memory optimization, concurrency management

## 📊 Monitoring and Validation

Each challenge includes validation functionality that checks if your solution correctly implements the required optimization. Always use the frontend UI and the "Validate your solution" button for both local and cloud validation. The UI provides immediate feedback and error details.

The validation process performs syntactic checks of your implementation. It verifies that your code genuinely implements the required solution while respecting the architectural constraints.

## 🎓 Learning Objectives

By completing these challenges, you'll learn:

1. **Asynchronous I/O**: How to properly use async/await for non-blocking file operations
2. **Database Query Optimization**: Techniques for writing efficient database queries and using appropriate indexing
3. **Memory Management**: Best practices for reducing allocations and using static references in Rust
4. **Resource Management**: How to properly manage and reuse expensive resources like HTTP clients


## 🏁 Conclusion

The ShellCon Smart Aquarium System is designed to provide a hands-on learning experience with real-world optimization challenges. By solving these challenges, you'll gain valuable experience with Rust backend development and Shuttle deployment.

Remember these key principles:

1. **Keep It Simple**: Focus on straightforward, effective solutions
2. **Verify Your Work**: Always test your solutions with the validation endpoints
3. **Check Logs**: Use `shuttle logs` to troubleshoot issues
4. **Format and Check**: Run `cargo fmt` and `cargo check` before deploying

Good luck, Rustacean! The crustaceans of ShellCon are counting on you! 🦀


## ❓ FAQ & Common Issues

**Q: I get an error that a port is already in use when running a service locally. What do I do?**

A: Free the port with:
```sh
lsof -ti :<port> | xargs kill -9
```

**Q: Shuttle asks me to log in when I deploy, interrupting my workflow.**

A: Always run `shuttle login` right after installing the CLI, before starting any work.

**Q: My service isn't picking up environment variables.**

A: Make sure you have set the variables in your shell _before_ running `shuttle run` or `shuttle deploy`, or use a `.env` file in the service directory.

**Q: How do I test endpoints locally?**

A: Use Thunder Client (VS Code), curl, or Postman. Example:
```sh
curl http://localhost:8000/api/health
```

**Q: How do I find my cloud endpoint after deploying?**

A: See the "Deploy to Shuttle Cloud" section above for instructions on updating your frontend and validating against cloud endpoints.

**Q: The validation endpoint says my solution is incorrect, but it works locally!**

A: Double-check that you followed the full validation workflow in the main instructions above:
- Validated locally using the frontend UI and `shuttle run`
- Deployed the latest code to Shuttle Cloud
- Updated your frontend to point to the correct cloud endpoints
- Checked Shuttle logs for errors (`shuttle logs --latest`)

**Q: I get a database connection error locally or an error about Docker socket.**

A: Make sure Docker is running on your machine. Shuttle uses Docker to provision PostgreSQL databases locally. If you see errors like `Socket not found: /var/run/docker.sock`, try these steps:

1. Start Docker Desktop or your Docker daemon
2. **In Docker Desktop settings, enable "Allow the default Docker socket to be used" option** (requires password)
3. Ensure your user has permissions to access the Docker socket
4. Restart the terminal where you're running Shuttle commands
5. If using VS Code devcontainer, ensure Docker-in-Docker is properly configured

**Q: How do I restart a service after code changes?**

A: Stop the running process (Ctrl+C) and re-run `shuttle run --port <port>` in the relevant directory.

**Q: Services aren't talking to each other.**

A: This is by design! Each service is independent. Only the frontend (when implemented) will coordinate between them.

If you have a question not answered here, check the Shuttle documentation or open an issue in this repository.
