# Sahels Platform

Sahels is a clinic operations platform tailored to the Saudi market. The project is structured as a monorepo with a Vue 3 frontend and a Rust Axum backend, designed for appointment scheduling, no-show reduction, AI-assisted workflows, messaging automation, and secure clinic administration.

## Architecture

- Frontend: Vue 3 + Vite + Tailwind CSS + shadcn-style UI
- Backend: Rust + Axum + Tokio
- Database: PostgreSQL
- Cache/Queues: Redis + BullMQ
- Security: JWT, rate limiting, password hashing, encryption-at-rest patterns, secure headers
- Observability: logs, request tracing, health checks

## Core modules

- Patient management
- Appointment scheduling and calendar management
- Reminder engine via WhatsApp / SMS
- Manual and automated follow-up flows
- No-show prediction and risk scoring
- Clinic staff dashboard and reports
- Multi-branch and multi-doctor support
- Booking and payment workflows

## Security standards

This repository intentionally includes security-first patterns for production readiness:

- rate limiting per IP and user
- secure password hashing with Argon2
- encryption helpers for secret values
- secure CORS and security headers
- environment-based configuration
- JWT-ready auth layer
- audit logging hooks

## Project structure

```text
.
├── apps
│   ├── api
│   │   └── src
│   └── web
│       ├── src
│       ├── index.html
│       └── package.json
├── docker-compose.yml
├── .env.example
├── .gitignore
└── README.md
```

## Quick start

### 1) Start infrastructure

```bash
docker compose up -d postgres redis
```

### 2) Backend

```bash
cd apps/api
cargo run
```

### 3) Frontend

```bash
cd apps/web
npm install
npm run dev
```

## Default URLs

- Frontend: http://localhost:5173
- Backend API: http://localhost:8080
- PostgreSQL: localhost:5432
- Redis: localhost:6379

## Notes

This is a production-minded starter scaffold, not a full production deployment. Use it as the base for a modular clinic platform with security-aware engineering from day one.
