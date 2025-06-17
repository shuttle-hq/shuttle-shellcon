# 🦀 ShellCon Smart Aquarium – Full-Stack Challenge Guide 🐚

Welcome to **ShellCon**, an onboarding playground where you will repair a broken _Smart Aquarium_ while learning Rust and Shuttle Cloud.  
When you launch the dashboard, read the short *Scenario* banner at the top. It sets the story context and your mission before you start clicking around.

This **monorepo** contains everything:

* Three Rust micro-services (`aqua-monitor`, `species-hub`, `aqua-brain`).
* A React + Vite dashboard (`frontend/`).

Your mission has **three phases**:

1. **Get it running** – start every service & the UI locally; verify every dashboard button works.
2. **Fix performance bugs** – complete four optimisation challenges inside the services.
3. **Ship to the cloud & celebrate** – deploy with Shuttle.

---
## 📂 Repo layout

```
services/
  aqua-monitor/   # environmental sensors  (Challenges 1 & 4)
  species-hub/    # species DB             (Challenge 2)
  aqua-brain/     # analytics engine       (Challenge 3)
frontend/         # React dashboard
```

---
## 🛠️ Prerequisites

* Rust 1.70+ & Cargo
* Docker Desktop (Postgres for local Shuttle)
* [Shuttle CLI](https://docs.shuttle.dev) (latest)
* Node ≥16 & npm (or Yarn/Bun)

### 🔑 One-time Shuttle setup
Install or update the Shuttle CLI, then **log in** before running any `shuttle run` or `shuttle deploy` commands.

```bash
# Install / update the CLI
curl -sSfL https://www.shuttle.dev/install | bash

# Authenticate (opens a browser for sign-in or account creation)
shuttle login
```

---
## 🚀 Quick-start – **Local**

### 1 · Launch the back-end services
Open three terminals:
```bash
# Terminal 1
cd services/aqua-monitor && shuttle run --port 8000
# Terminal 2
cd services/species-hub  && shuttle run --port 8001
# Terminal 3
cd services/aqua-brain  && shuttle run --port 8002
```
If a port is busy: `lsof -ti :<port> | xargs kill -9` then retry.

### 2 · Start the dashboard & confirm every API call locally
```bash
cd frontend
npm install                 # first time only
#If ports 8000-8002 are not available on your machine, check the "Environment files (frontend)" section below to customize the ports. 
npm run dev:localhost        # opens http://localhost:8080
```
In the **System Control Panel** click each button – no red errors should appear.

| UI action | Request | Service |
|-----------|---------|---------|
| View all tanks | GET `/api/tanks` | aqua-monitor |
| Tank readings  | GET `/api/tanks/{id}/readings` | aqua-monitor |
| Sensor status  | GET `/api/sensors/status` | aqua-monitor |
| List species   | GET `/api/species` | species-hub |
| Species details | GET `/api/species/{id}` | species-hub |
| Feeding schedule | GET `/api/species/{id}/feeding-schedule` | species-hub |
| All tank analysis | GET `/api/analysis/tanks` | aqua-brain |
| Single tank analysis | GET `/api/analysis/tanks/{id}` | aqua-brain |

> Challenges only affect **performance/validation** – the endpoint contracts above stay identical before & after solving them.

---
### 🖥️ Frontend dashboard tour

The dashboard is split into five panels:

| Panel | What it shows |
|-------|---------------|
| **Scenario Banner** (top) | A short narrative that frames why youre fixing the aquarium – read it first! |
| **System Control Panel** | Action buttons that call the REST endpoints listed above. |
| **System Status** | Live backend connectivity and challenge status shown as **Normal** or **Error/Degraded**. |
| **Optimization Challenges**| Cards describing each challenge with _Show Hint_, _Validate_, _View Solution_, and _Teach Me How It Works_ buttons. |

After all challenges pass, every service status should switch to **Normal** and a confetti animation will celebrate your victory.

#### Key UI capabilities
* **Interactive Challenge Cards** – hint, validate, full solution, plus status badge.
* **System Visualization & Status** – live backend connectivity and tank health.
* **Persistent state** – challenge completion and status are stored in `localStorage` so they survive page reloads.
* **Rich Log Console** – real-time stream of API requests and validator output.

---
### ⚙️ Environment files – local development (`.env.localhost`)

`vite.config.ts` looks for three variables but falls back to the default localhost ports 8000-8002:

```
VITE_AQUA_MONITOR_URL    (default http://localhost:8000)
VITE_SPECIES_HUB_URL     (default http://localhost:8001)
VITE_AQUA_BRAIN_URL      (default http://localhost:8002)
```

You override them by specifying other ports, as shown in the example below.

| File | When Vite picks it up | Typical use |
|------|----------------------|-------------|
| `.env.localhost` | `npm run dev:localhost` | Custom local ports or running services on another host. |

Example **.env.localhost** (custom ports):
```env
# Local development backend services
VITE_AQUA_MONITOR_URL=http://localhost:8020
VITE_SPECIES_HUB_URL=http://localhost:8021
VITE_AQUA_BRAIN_URL=http://localhost:8022
VITE_API_BASE_URL=/api  # leave as /api unless you modify the proxy rules
```

Example **.env.prod** (cloud):
```env
VITE_AQUA_MONITOR_URL=https://aqua-monitor-xxxxx.shuttleapp.app
VITE_SPECIES_HUB_URL=https://species-hub-xxxxx.shuttleapp.app
VITE_AQUA_BRAIN_URL=https://aqua-brain-xxxxx.shuttleapp.app
VITE_API_BASE_URL=/api
```

Steps:
1. `touch frontend/.env.localhost` and adjust if needed.
2. Restart the dev server whenever you edit an `.env*` file.

---
### 🌐 Environment files – cloud deployment (`.env.prod`)
Create `frontend/.env.prod` **after** you deploy each service with Shuttle.

The file should contain:
```env
# where <aqua-monitor> etc. are your Shuttle project IDs (shown after deploy)
VITE_AQUA_MONITOR_URL=https://<aqua-monitor>.shuttleapp.app   # <aqua-monitor> is your project ID
VITE_SPECIES_HUB_URL=https://<species-hub>.shuttleapp.app     # <species-hub> is your project ID
VITE_AQUA_BRAIN_URL=https://<aqua-brain>.shuttleapp.app       # <aqua-brain> is your project ID
VITE_API_BASE_URL=/api
```
Run the dashboard against the cloud:
```bash
npm run dev:prod
```

---

### 3 · Solve the optimisation challenges
| # | Location | Topic |
|---|----------|-------|
| 1 | `aqua-monitor/src/challenges.rs::get_tank_readings` | Async I/O |
| 2 | `species-hub/src/challenges.rs::get_species` | SQL optimisation |
| 3 | `aqua-brain/src/challenges.rs::get_analysis_result` | String allocations |
| 4 | `aqua-monitor/src/challenges.rs::get_sensor_status` | HTTP-client reuse |

Hints & full write-ups live in `services/aqua-brain/src/lectures/`.

Workflow per challenge:
1. Read the lecture / hint.
2. Edit the code inside the marked `// ⚠️ CHALLENGE` block.
3. Re-run **only** that service (`Ctrl-C`, then `shuttle run --port …`).
4. Click **Validate** in the UI.

Complete all four – the UI unlocks confetti! 🎉

---
<!-- FAQ section moved below -->

**Q: Shuttle complains I’m not logged in.**  
A: Run `shuttle login` once (see “One-time Shuttle setup”).

**Q: A port is already in use.**  
A: `lsof -ti :<port> | xargs kill -9` frees it before re-running.

**Q: The UI shows red banners / fetch errors.**  
A: Check that each backend terminal shows `Listening on 0.0.0.0:PORT` and that your `.env*` URLs match.

**Q: Validation keeps failing even after I fixed the code.**  
A: Restart the corresponding service, then click *Validate* again; the validator inspects live code.

**Q: How do I reset a service database?**  
A: Stop the service, delete `services/<service>/.local-data`, and rerun `shuttle run --port …`.

---
## ☁️ Deploy & validate in the **cloud**

1. For each service:
   ```bash
   cd services/<service>
   shuttle deploy                # choose “CREATE NEW”, project name = folder name
   ```
2. Configure the **frontend for cloud**:
   ```bash
   # create the env file once (or edit it if it already exists)
   touch frontend/.env.prod
   ```
   Open `frontend/.env.prod` and paste the URLs printed by each `shuttle deploy` command:

   ```env
   VITE_AQUA_MONITOR_URL=https://<aqua-monitor>.shuttleapp.app
   VITE_SPECIES_HUB_URL=https://<species-hub>.shuttleapp.app
   VITE_AQUA_BRAIN_URL=https://<aqua-brain>.shuttleapp.app
   VITE_API_BASE_URL=/api
   ```

3. Restart the dashboard pointed at the cloud back-end:
   ```bash
   cd frontend
   npm run dev:prod
   ```
   Re-validate each challenge card to confirm remote success.

## ❓ FAQ

**Q: Shuttle complains I’m not logged in.**  
A: Run `shuttle login` once (see “One-time Shuttle setup”).

**Q: A port is already in use.**  
A: `lsof -ti :<port> | xargs kill -9` frees it before re-running.

**Q: The UI shows red banners / fetch errors.**  
A: Check that each backend terminal shows `Listening on 0.0.0.0:PORT` and that your `.env*` URLs match.

**Q: Validation keeps failing even after I fixed the code.**  
A: Restart the corresponding service, then click *Validate* again; the validator inspects live code.

**Q: How do I reset a service database?**  
A: Stop the service, delete `services/<service>/.local-data`, and rerun `shuttle run --port …`.

---
## 🩹 Troubleshooting cheat-sheet

* `cargo check` + `cargo fmt` before running.
* Runtime errors: `shuttle logs --latest`.
* Validation fails?  Ensure the UI points to the right environment (localhost vs cloud).
* Port in use: `lsof -ti :<port> | xargs kill -9`.

---
## 🎓 What you’ll learn

* Async vs blocking I/O in Rust
* Writing efficient SQL with SQLx & Postgres
* Minimising heap allocations (Strings & Vec)
* Resource pooling & shared state with Axum
* Shuttle local dev → cloud deployment workflow

Good luck, Rustacean – the aquarium (and Ferris the crab 🦀) is counting on you!
