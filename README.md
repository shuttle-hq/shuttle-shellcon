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
1. Clone the frontend repository
2. Follow the setup instructions in its README
3. Configure it to point to your deployed services

## 🚀 Getting Started with Shuttle Cloud

Shuttle is a platform that makes deploying Rust applications simple. Unlike traditional deployment methods, Shuttle handles infrastructure for you, letting you focus on writing Rust code.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or newer)
- [Shuttle CLI](https://docs.shuttle.rs/getting-started/installation) (latest version)

### Installing Shuttle CLI

If you don't have a Shuttle account, please create one [here](https://console.shuttle.dev/login).

```bash
# Install the Shuttle CLI
curl -sSf https://docs.shuttle.rs/install.sh | bash

# Log in to Shuttle
shuttle login
```

## 🛠️ Deploying to Shuttle Cloud

Rather than running services locally, we'll deploy directly to Shuttle Cloud. This approach allows you to work with a production-like environment from the start.

### 1. Preparing Your Code for Deployment

Before deploying, always ensure your code compiles and is properly formatted:

```bash
# Format your code according to Rust standards
cd services/aqua-monitor
cargo fmt

# Check for compilation errors without building
cargo check
```

Repeat these steps for each service you modify. **Always fix any compilation errors before deploying** to avoid wasting time on failed deployments.

### 2. Deploying the Services

Deploy each service to Shuttle Cloud in this recommended order:

```bash
# First, deploy species-hub (database service)
cd services/species-hub
shuttle deploy

# Next, deploy aqua-monitor
cd ../aqua-monitor
shuttle deploy

# Finally, deploy aqua-brain
cd ../aqua-brain
shuttle deploy
```

After each successful deployment, Shuttle will display the unique URL for your service, such as:
- `https://aqua-brain-xyz123.shuttle.app`
- `https://aqua-monitor-abc456.shuttle.app`
- `https://species-hub-def789.shuttle.app`

**Save these URLs** - you'll need them to test your services and validate challenge solutions.

> **Important**: First-time deployments may take 3-5 minutes to fully initialize. Be patient if services aren't immediately responsive.

### 3. Testing Your Deployed Services

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

### 4. Redeploying After Changes

As you solve challenges, you'll need to redeploy your services to apply your changes:

```bash
# After making changes to fix a challenge
cd services/aqua-monitor

# Format and check your code
cargo fmt
cargo check

# Redeploy to Shuttle Cloud
shuttle deploy

# Check logs to verify deployment
shuttle logs --latest
```

After redeploying, test your solution using the validation endpoint:

```bash
# Example: Validating Challenge 4 solution
curl https://aqua-monitor-abc456.shuttle.app/api/challenges/4/validate
```
> **Note**: You can also test locally using `shuttle run`, but don't forget to restart your frontend using `npm run dev:localhost` to reflect the changes in the UI.

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
```

### 4. Deploy Your Solution

```bash
# Deploy your changes
shuttle deploy
```
> **Note**: You can also test locally using `shuttle run`, but don't forget to restart your frontend using `npm run dev:localhost` to reflect the changes in the UI.

### 5. Validate Your Solution

You can validate your solution in two ways:

1. **Using the UI (Recommended)**:
   - Navigate to the challenge in the frontend
   - Click the "Validate Solution" button
   - Get immediate visual feedback and detailed error messages if any

2. **Using the API directly**:
   ```bash
   # Test the validation endpoint
   curl https://your-service-url.shuttle.app/api/challenges/1/validate
   ```

   A successful validation will return a JSON response with `"valid": true`.

   > **Note**: Replace `your-service-url` with your actual Shuttle deployment URL and the challenge number with the one you're working on (1-4).
   > For local testing, use `http://localhost:<port>` with the appropriate port number for each service:
   > - aqua-monitor: 8000
   > - species-hub: 8001
   > - aqua-brain: 8002

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

Each challenge includes a validation endpoint that checks if your solution correctly implements the required optimization:

```bash
# Example: Validating Challenge 4 (The Leaky Connection)
curl https://your-aqua-monitor-url.shuttle.app/api/challenges/4/validate
```

The validation endpoints perform syntactic checks of your implementation. They verify that your code genuinely implements the required solution while respecting the architectural constraints.

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
