# Sahels Platform

Sahels is a clinic operations platform for the Saudi market, designed to reduce no-shows, automate appointment communication, and centralize multi-clinic administration.

This version reflects the product direction discussed earlier and aligns with the original Sahels repository structure, while upgrading the stack around Vue 3 + Tailwind + Rust + PostgreSQL + Redis and adding stronger operational and security features.

## Included in this version

- Clinic login and registration UI
- Clinic/doctor appointment models
- Reminder and message workflow scaffold
- PostgreSQL schema aligned with clinic operations and tenant isolation
- Tokenized per-clinic integration model for payment, WhatsApp, and Instagram
- OTP and secure authentication scaffolding
- Backup and restore automation scripts
- Security-first configuration for production use

## Stack

- Frontend: Vue 3 + Vite + Tailwind CSS + shadcn-style UI
- Backend: Rust + Axum + Tokio
- Database: PostgreSQL
- Cache / queue: Redis
- Security: JWT, Argon2, encryption helpers, rate limiting, secure headers, OTP hooks

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
├── docs/
├── scripts/
├── docker-compose.yml
├── .env.example
├── .gitignore
├── init.sql
├── README.md
└── package.json
```

## Security and compliance notes

- clinic-level tokens for payment, WhatsApp, Instagram, OTP, and API access
- rate limiting on auth and API routes
- secret encryption helpers
- backup retention automation
- environment-based configuration for production deployments
- Saudi-market-aligned tenant model and clinic isolation hooks

## Quick start

```bash
docker compose up -d postgres redis

cd apps/api
cargo run

cd ../web
npm install
npm run dev
```

## Default ports

- Frontend: http://localhost:5173
- API: http://localhost:8080
- PostgreSQL: localhost:5432
- Redis: localhost:6379

## Notes

This version is a production-minded MVP foundation, not a final deployment. It is designed to evolve into a multi-clinic, secure operations platform with reminder automation, payment orchestration, clinic-level tokens, and OTP-driven validation.
