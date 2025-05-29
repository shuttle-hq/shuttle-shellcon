# 🦀 ShellCon Smart Aquarium System 🦀

Welcome to the ShellCon Smart Aquarium System! This interactive project will introduce you to building high-performance microservices with Rust and Shuttle Cloud.

## 🌊 The ShellCon Scenario

Imagine you've just joined the emergency technical response team for ShellCon, the world's premier convention for Rustaceans and crustaceans alike! This year's main attraction is a revolutionary Smart Aquarium system built with Rust and deployed on Shuttle.

**The problem?** Just hours before the convention opens, several performance issues have been detected in the backend services. As the newly recruited Rustacean engineer, you've been called in to optimize these systems before the doors open to the public.

The convention organizers are in a pinch—quite literally, as the convention's mascot, a giant Coconut Crab named Ferris, is anxiously clicking his claws at the mounting technical issues!

## 🏗️ System Architecture

The Smart Aquarium System consists of three microservices:

- **aqua-monitor**: Collects real-time environmental data from tank sensors
- **species-hub**: Manages the species database and feeding requirements
- **aqua-brain**: Performs analysis and coordinates system responses

## 🚀 Getting Started with Shuttle Cloud

Shuttle is a platform that makes deploying Rust applications simple. Unlike traditional deployment methods, Shuttle handles infrastructure for you, letting you focus on writing Rust code.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or newer)
- [Shuttle CLI](https://docs.shuttle.rs/getting-started/installation) (latest version)

### Installing Shuttle CLI

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

### 3. Checking Deployment Logs

If you encounter issues or want to verify your service is running correctly, check the logs:

```bash
# View the latest logs for a service
cd services/aqua-monitor
shuttle logs --latest
```

### 4. Testing Your Deployed Services

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

### 5. Redeploying After Changes

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

## CORS Configuration

The backend services are already configured to accept requests from any origin during development. For production, you may want to restrict this to only your frontend domain.

If you need to customize CORS settings, look for the CORS middleware configuration in each service's `main.rs` file.

## 🕹️ The Optimization Challenges

Your mission is to solve five performance challenges across the microservices. Each challenge focuses on a different aspect of backend optimization in Rust.

### Challenge 1: The Sluggish Sensor (Async I/O)
- **Service**: aqua-monitor
- **File**: src/main.rs
- **Function**: get_tank_readings
- **Problem**: The environmental monitoring system is experiencing severe delays due to inefficient file I/O operations.

### Challenge 2: The Query Conundrum (Database Optimization)
- **Service**: species-hub
- **File**: src/main.rs
- **Function**: get_species
- **Problem**: The species database is responding slowly to searches due to inefficient queries.

### Challenge 3: The Memory Miser (String Optimization)
- **Service**: aqua-brain
- **File**: src/main.rs
- **Function**: get_analysis_result
- **Problem**: The analysis engine is consuming excessive memory due to inefficient string handling.

### Challenge 4: The Leaky Connection (Resource Management)
- **Service**: aqua-monitor
- **File**: src/main.rs
- **Function**: get_sensor_status
- **Problem**: The sensor status API is creating a new HTTP client for every request, causing resource leaks.

### Challenge 5: Safe Shared State (Concurrency)
- **Service**: aqua-brain
- **File**: src/main.rs
- **Function**: shared_state_example
- **Problem**: Unsafe shared state is causing data races or panics in the analysis engine.

## 🧰 How to Solve a Challenge

Follow this workflow to solve each challenge:

### 1. Understand the Problem

Examine the challenge description and the problematic code:

```bash
# View the source code for the challenge
cat services/aqua-monitor/src/main.rs | grep -A 20 "get_sensor_status"
```

### 2. Implement Your Solution

Edit the code to fix the performance issue. For example, to solve Challenge 4:

```rust
// Add at the top of the file
use once_cell::sync::Lazy;

// Define a static HTTP client
static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());

// In the get_sensor_status function, replace:
// let client = reqwest::Client::new();
// with:
let client = &*HTTP_CLIENT;
```

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

### 5. Validate Your Solution

```bash
# Test the validation endpoint
curl https://your-service-url.shuttle.app/api/challenges/4/validate
```

A successful validation will return a JSON response with `"valid": true`.

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

### Challenge 5: Concurrency
- Look for shared state that needs thread-safe access
- Consider using Tokio's async-aware synchronization primitives

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

```bash
# Check logs after validation
shuttle logs --latest
```

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

The validation endpoints perform real checks of your implementation - they don't just simulate success. They verify that your code genuinely implements the required solution while respecting the architectural constraints.

## 🎓 Learning Objectives

By completing these challenges, you'll learn:

1. **Performance Optimization**: Practical techniques for making Rust services faster and more efficient
2. **Resource Management**: How to properly handle expensive resources in web services
3. **Concurrency Patterns**: Safe approaches to shared state in async Rust
4. **Database Efficiency**: Optimizing database queries for better performance
5. **Shuttle Deployment**: How to deploy and manage Rust services in the cloud

## 🏁 Conclusion

The ShellCon Smart Aquarium System is designed to provide a hands-on learning experience with real-world optimization challenges. By solving these challenges, you'll gain valuable experience with Rust backend development and Shuttle deployment.

Remember these key principles:

1. **Keep It Simple**: Focus on straightforward, effective solutions
2. **Verify Your Work**: Always test your solutions with the validation endpoints
3. **Check Logs**: Use `shuttle logs` to troubleshoot issues
4. **Format and Check**: Run `cargo fmt` and `cargo check` before deploying

Good luck, Rustacean! The crustaceans of ShellCon are counting on you! 🦀
