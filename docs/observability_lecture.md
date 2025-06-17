# Lecture: Observability with Rust, OpenTelemetry, and Shuttle

This document explains how Rust, OpenTelemetry (OTEL), and Shuttle work together to provide powerful observability (tracing, logs, and spans) for your distributed application.

## The Goal: Understanding Your System's Story

In a microservices architecture, a single user action (like checking sensor data) can trigger a chain of requests across multiple services (`aqua-monitor` -> `aqua-brain` -> `species-hub`). The end goal of observability is to trace this entire journey from start to finish.

We want to answer questions like:
- What was the full path of this request through all services?
- How long did each service take to process its part?
- If an error occurred in `species-hub`, what was the original request in `aqua-monitor` that caused it?
- What specific `tank_id` was being processed at each step?

This unified view is critical for debugging, performance tuning, and understanding how your system behaves in the real world.

---

## The Players

### 1. Rust and the `tracing` Crate

- **What it is**: `tracing` is the standard library in the Rust ecosystem for instrumenting applications. It's not just for simple logging; it's for structured, context-aware diagnostics.
- **How it works**: It introduces two key concepts:
    - **Spans**: Represent a period of time with a start and an end (e.g., an HTTP request, a database query). Spans can be nested to create a hierarchy.
    - **Events**: Represent a single point in time (e.g., a log message, an error). Events occur *within* the context of the current span.
- **The Magic**: When you log an event inside a span, that log is automatically tagged with the span's context (like a `request_id`). This is how you connect disparate log messages into a single, coherent story.

### 2. OpenTelemetry (OTEL)

- **What it is**: OTEL is a vendor-neutral, open-source standard for collecting telemetry data (traces, metrics, logs). It provides a set of APIs and SDKs so you can instrument your code once and send the data to *any* compatible backend (like Jaeger, Datadog, or Shuttle's own platform).
- **How it works**: An OTEL-compatible tool (an "exporter") runs alongside your application. It collects the telemetry data and sends it to a backend for storage and analysis.
- **The Goal**: To free you from vendor lock-in. You write your instrumentation code against the OTEL standard, not a specific vendor's proprietary agent.

### 3. Shuttle: The Infrastructure Provider

- **What it is**: Shuttle is an "Infrastructure from Code" platform. It provisions and manages the necessary cloud infrastructure based on your code.
- **How it works with OTEL**: Shuttle embraces this philosophy for observability. When you enable a feature flag in your `Cargo.toml`, you are telling Shuttle, "My application needs an observability backend."
    - `shuttle-runtime = { features = ["setup-otel-exporter"] }`
- **The Magic**: With this one line, Shuttle does all the heavy lifting:
    1.  It injects an OTEL-compatible exporter into your deployed service.
    2.  It automatically configures your application's `tracing` library to send all its data (spans and events) to this exporter.
    3.  It provisions and manages the backend infrastructure to receive, process, and store this data.
    4.  It provides you with a dashboard to view and query your traces and logs.

You don't have to configure collectors, manage databases, or set up agents. You just write `tracing` code and let Shuttle handle the rest.

---

## Summary: The Workflow

1.  **You write Rust code** using the `tracing` crate to describe what your application is doing (`info_span!`, `info!`, etc.).
2.  **You tell Shuttle** you want observability by enabling the `setup-otel-exporter` feature.
3.  **Shuttle connects the dots**, ensuring your `tracing` data is exported in the OTEL format to its managed backend.
4.  **You get a powerful, unified view** of your application's behavior in the Shuttle console, allowing you to trace requests across all your services.
