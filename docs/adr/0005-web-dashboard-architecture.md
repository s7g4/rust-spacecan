# ADR 0005: Web Dashboard Architecture

## Status
Accepted

## Context
Monitoring spacecraft telemetry and dispatching ECSS PUS telecommands via command-line interface (CLI) binaries is cumbersome, non-intuitive, and prone to user error. To elevate the project to an industry-grade standard, operators require a visual Mission Control interface.

## Decision
We will introduce a **Web-Based Ground Station Dashboard**.
1. **Frontend**: A React SPA built with Vite (`dashboard/`), utilizing a modern, glassmorphism design system independent of heavy CSS frameworks like Tailwind.
2. **Backend Gateway**: An Axum-based WebSocket server (`spacecan-virtual/dashboard_server.rs`) that proxies traffic between the React frontend and the UDP Multicast simulation network.

## Consequences
- **Positive**: Operators can visually monitor real-time scrolling telemetry and click intuitive buttons to dispatch complex ST01/ST03/ST08 commands.
- **Positive**: Separates the visualization logic from the protocol routing logic.
- **Negative**: Introduces Node.js/npm dependencies to the repository solely for building the frontend assets.
