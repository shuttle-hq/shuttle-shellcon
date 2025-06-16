# 🦀 ShellCon Frontend - Smart Aquarium Dashboard 🐚

## 🔍 Overview

This repository contains the frontend application for the ShellCon Smart Aquarium Dashboard. It's designed to work with three Rust backend microservices running on Shuttle. The application provides an interactive interface for solving shell scripting challenges in the context of managing a smart aquarium system. The frontend uses React with Vite and communicates with the backend services via REST APIs.

## 📋 Prerequisites

- Node.js 16+ and npm
- Rust and Cargo installed
- Shuttle CLI installed (`curl -sSfL https://www.shuttle.dev/install | bash`)
- Git for cloning repositories
- **Backend Repository**: [https://github.com/shuttle-hq/shuttle-shellcon](https://github.com/shuttle-hq/shuttle-shellcon)

## 🔧 Backend Services

The frontend communicates with three separate backend microservices:

1. **Aqua Monitor** (Port 8000): Environmental monitoring system and challenge #1 validation
2. **Species Hub** (Port 8001): Species database management and challenge #2 validation
3. **Aqua Brain** (Port 8002): System status reporting and challenges #3-4 validation

## 💻 Getting Started with Local Development

## Step 1: Set Up the Backend Services Locally

Follow the instructions in the [backend repository](https://github.com/shuttle-hq/shuttle-shellcon) to set up the backend services locally.

## Step 2: Set Up the Frontend

1. Clone this frontend repository (in a new terminal):
   ```bash
   git clone https://github.com/shuttle-hq/shuttle-shellcon-frontend.git
   cd shuttle-shellcon-frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Create a `.env.localhost` file for local development:
   ```bash
   # Create the .env.localhost file (this file is in .gitignore and won't exist in the cloned repo)
   touch .env.localhost
   ```

4. Add the local backend URLs to the `.env.localhost` file:
   ```env
   VITE_AQUA_MONITOR_URL=http://localhost:8000
   VITE_SPECIES_HUB_URL=http://localhost:8001
   VITE_AQUA_BRAIN_URL=http://localhost:8002
   VITE_API_BASE_URL=/api
   ```

5. Start the frontend with the local configuration:
   ```bash
   npm run dev:localhost
   ```

6. Access the application at `http://localhost:8080`

## Step 3: Test All System Control Panel Actions (Pre-Challenge Checklist)

Before starting the challenges, your first task is to verify that all buttons and system control actions in the ShellCon Smart Aquarium System Control Panel work correctly.

This ensures your frontend is communicating with the backend services and that your environment is set up properly. **Explore each button and action in the control panel UI. If the backend is running, you should NOT see any error messages in the panel.**

Below is a mapping of each control panel button to its backend API call and service:

| Button / Action                 | API Endpoint & Method                                  | Backend Service    |
|----------------------------------|--------------------------------------------------------|--------------------|
| View All Tanks                   | `GET /tanks`                                           | Aqua Monitor (8000)|
| Fetch Tank Readings (by tank)    | `GET /tanks/{tankId}/readings`                         | Aqua Monitor (8000)|
| Check Sensor Status              | `GET /sensors/status`                                  | Aqua Monitor (8000)|
| List All Species                 | `GET /species`                                         | Species Hub (8001) |
| Get Species Details (by species) | `GET /species/{speciesId}`                             | Species Hub (8001) |
| Get Feeding Schedule (by species)| `GET /species/{speciesId}/feeding-schedule`            | Species Hub (8001) |
| View All Tank Analysis           | `GET /analysis/tanks`                                  | Aqua Brain (8002)  |
| Tank Health Analysis (by tank)   | `GET /analysis/tanks/{tankId}`                         | Aqua Brain (8002)  |

**How to test:**
- Click each button in the control panel and verify that data loads without error.
- Use dropdowns/selectors where applicable (e.g., tank or species selection) and confirm the correct API call is made.
- If you see any error message or no data loads, double-check that the backend service for that action is running and accessible on the correct port.

**Only proceed to the challenges once all actions above work without errors.**

### Exploring the Backend API Implementation

All API endpoints are implemented in Rust using the Axum web framework. You can find the source code for each service in the backend repository under `services/<service-name>/src/main.rs`. Each service defines its routes in the `shuttle_runtime::main` function.

For example, here's how the Aqua Monitor service (`services/aqua-monitor/src/main.rs`) defines its routes:

```rust
#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // Build router with all endpoints
    let router = Router::new()
        .route("/api/tanks", get(get_all_tanks))
        .route("/api/tanks/:tank_id/readings", get(challenges::get_tank_readings))
        .route("/api/sensors/status", get(challenges::get_sensor_status))
        // ... challenge validation routes
        .with_state(state);

    Ok(router.into())
}
```

To explore the API implementations:
1. Open your local backend repository folder
2. Navigate to `services/<service-name>/src/main.rs`
3. Look for the `shuttle_runtime::main` function
4. Each `.route()` call maps an endpoint to its handler function
5. Follow the handler functions to understand the implementation

The three services are structured similarly:
- **Aqua Monitor**: `services/aqua-monitor/src/main.rs`
- **Species Hub**: `services/species-hub/src/main.rs`
- **Aqua Brain**: `services/aqua-brain/src/main.rs`

---

## Step 4: Solve the Challenges Locally

Work through the challenges using your local environment. This allows you to:
- Make quick iterations to your solutions
- Test changes immediately
- Debug more effectively

## 🚀 Deploying to Shuttle Cloud

Once you've completed the challenges locally, you can deploy to Shuttle Cloud:

## Step 5: Deploy Backend Services to Shuttle Cloud

1. Follow the instructions from the backend repository to deploy to Shuttle Cloud.

2. After successful deployment, Shuttle will provide URLs for each service. Note these URLs as you'll need them for the frontend configuration.

## Step 6: Configure the Frontend for Cloud Backend

1. Create a `.env.prod` file in your frontend project:
   ```bash
   # Create the .env.prod file (this file is in .gitignore and won't exist in the cloned repo)
   touch .env.prod
   ```

2. Add your Shuttle-deployed backend URLs to the `.env.prod` file:
   ```env
   # Replace with your actual Shuttle-deployed service URLs
   VITE_AQUA_MONITOR_URL=https://your-aqua-monitor-service.shuttleapp.app
   VITE_SPECIES_HUB_URL=https://your-species-hub-service.shuttleapp.app
   VITE_AQUA_BRAIN_URL=https://your-aqua-brain-service.shuttleapp.app
   VITE_API_BASE_URL=/api
   ```
   
   > **Note**: After deploying with `shuttle deploy`, you'll receive the actual URLs for your services in the terminal output. Use these URLs in your `.env.prod` file.

3. Start the frontend with the production configuration:
   ```bash
   npm run dev:prod
   ```

4. Access the application at `http://localhost:8080` and verify all services are connected properly

## 🔄 Switching Between Local and Cloud Backends

## To Use Local Backend

1. Start the backend services locally:
   ```bash
   cd shuttle-shellcon
   shuttle run
   ```

2. Run the frontend with local configuration:
   ```bash
   npm run dev:localhost
   ```

## To Use Cloud Backend

1. If not already deployed, follow the instructions from the backend repository to deploy to Shuttle Cloud.

2. Make sure your `.env.prod` file has the correct URLs:
   ```env
   VITE_AQUA_MONITOR_URL=https://your-aqua-monitor-service.shuttleapp.app
   VITE_SPECIES_HUB_URL=https://your-species-hub-service.shuttleapp.app
   VITE_AQUA_BRAIN_URL=https://your-aqua-brain-service.shuttleapp.app
   VITE_API_BASE_URL=/api
   ```

3. Run the frontend with the production configuration:
   ```bash
   npm run dev:prod
   ```

In both cases, the application will be available at `http://localhost:8080`.

## 🛠️ Troubleshooting

### Common Issues

1. **Backend Connection Errors**:
   - Verify that all three backend services are running (check the Shuttle dashboard or run `shuttle logs --latest`)
   - Ensure the URLs in your `.env.localhost` or `.env.prod` file are correct
   - Check the browser console for specific error messages
   - Make sure you're using the correct command (`npm run dev:localhost` for local backend, `npm run dev:prod` for cloud backend)

2. **CORS Issues**:
   - If you see CORS errors in the console, ensure the backend services are configured to accept requests from your frontend domain
   - For local development, the backend services should allow requests from `http://localhost:8080`
   - Try clearing your browser cache or using an incognito/private window

3. **Challenge Validation Problems**:
   - Check the backend logs for any errors: `shuttle logs --latest`
   - Verify that you're following the challenge instructions exactly as specified
   - Make sure you've completed all the prerequisites for each challenge
   - Check if your solution matches the expected format (case-sensitive, exact spacing, etc.)

4. **Frontend Loading Issues**:
   - If challenges don't load, check the Network tab in your browser's developer tools
   - Verify that the API requests are being sent to the correct URLs
   - Try restarting both the frontend and backend services

## ❓ Frequently Asked Questions (FAQ)

### General Questions

1. **Q: Do I need to deploy the frontend to a hosting service?**
   A: No, you can run the frontend locally using `npm run dev:localhost` (for local backend) or `npm run dev:prod` (for cloud backend).

2. **Q: Can I work on the challenges without deploying the backend to Shuttle Cloud?**
   A: Yes, you can run the backend locally using `shuttle run` and connect to it with `npm run dev:localhost`.

3. **Q: How do I know if my backend services are running correctly?**
   A: Use `shuttle logs --latest` to check the logs. You should see successful startup messages for all three services.

### Shuttle-Specific Questions

1. **Q: Where can I find deployment instructions for the backend?**
   A: Refer to the documentation in the backend repository for detailed deployment instructions.

2. **Q: How can I check if my Shuttle services are running in the cloud?**
   A: Visit the [Shuttle dashboard](https://console.shuttle.dev/) or check your project status in the Shuttle console.

3. **Q: How do I update my cloud deployment after making changes?**
   A: Follow the update instructions in the backend repository documentation.

### Challenge-Specific Questions

1. **Q: The challenge validation isn't working even though my solution seems correct. What should I check?**
   A: Verify that:
   - You're connected to the correct backend (local or cloud)
   - Your solution exactly matches the expected format
   - The backend logs don't show any validation errors
   - You've completed all prerequisites for the challenge

2. **Q: My challenge progress isn't being saved between sessions. Why?**
   A: The challenge progress is stored in your browser's localStorage. Try:
   - Using the same browser and device
   - Not clearing your browser data
   - Completing each challenge fully before moving to the next
   
   > **Note**: The application uses localStorage to persist challenge validation state and system status changes between page refreshes. This ensures that when a challenge is validated as solved, it remains in the solved state even after page reloads.

3. **Q: The code examples in the challenges aren't displaying correctly. How can I fix this?**
   A: If code blocks aren't rendering properly:
   - Try refreshing the page
   - Clear your browser cache
   - Try a different browser if the issue persists

## 🏆 Challenge Information

The challenges and their solutions are described in the backend repository. The frontend simply provides an interface for interacting with these challenges.

For details on each challenge and how to solve them, please refer to the documentation in the backend repository.

### Smart Aquarium Dashboard Features

The frontend application includes several key features:

1. **Interactive Challenge Cards**: Each challenge is presented as a card with description, validation status, and solution submission.
2. **System Status Visualization**: Real-time visualization of the aquarium system components and their status.
3. **Persistent State**: Challenge validation status and system component status are persisted in localStorage to maintain state between page refreshes.
4. **API Integration**: Seamless integration with the backend microservices for challenge validation and system monitoring.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Last updated: June 2025*
