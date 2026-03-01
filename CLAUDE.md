# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Pulsarr is a full-stack music rating and sharing platform for groups of users with different streaming subscriptions. It enables centralized music discussion, group rating/curation, and listening habit analytics.

**Stack**: Vue 3 + TypeScript frontend (`pulsarr_web/`) with Rust + Rocket backend (`pulsarr_fulcrum/`), PostgreSQL database.

## Common Commands

### Frontend (`pulsarr_web/`)
```bash
npm run dev              # Start Vite dev server
npm run build            # Production build with type checking
npm run type-check       # Run Vue TypeScript compiler
npm run generateModels   # Generate OpenAPI client from backend spec
```

### Backend (`pulsarr_fulcrum/`)
```bash
cargo build              # Build the Rust backend
cargo run                # Run the server (auto-runs migrations)
```

### Docker Setup
```bash
docker network create dokploy-network   # Required network for local dev
# Frontend: http://localhost:3003
# Backend API: http://localhost:4004
```

## Architecture

### Frontend (`pulsarr_web/src/`)
- **Vue 3 Composition API** with `<script setup>` pattern
- **Pinia store** (`stores/context.ts`) for global state (user session, API key, privacy/rating types)
- **Auto-generated API client** (`apiClient/`) from OpenAPI spec - run `npm run generateModels` after backend API changes
- **DataRequestHandler** (`helpers/DataRequestHandler.ts`) wraps fetch with API key injection via `pulsarr-api-key` header
- **ShadCN-Vue components** (`components/ui/`) built on Reka-UI
- **Vee-Validate + Yup** for form validation
- **Tailwind CSS v4** with CSS variables and class-based dark mode

### Backend (`pulsarr_fulcrum/src/`)
- **Rocket web framework** with async handlers and CORS
- **OpenAPI/Swagger** auto-generated at `/swagger/` via rocket-okapi
- **SQLx** with compile-time checked queries against PostgreSQL
- **Guard-based auth** (`api/guards/api_key.rs`) validates `pulsarr-api-key` header
- **DTO layer** (`api/dtos/`) separates API contracts from domain models with explicit conversion functions
- **DataWrangler** (`data/data_wrangler.rs`) provides generic CRUD operations

### API Endpoints
- `/auth` - Sign in/sign up
- `/user` - User management
- `/group` - Group CRUD, join, search, privacy types
- `/rating-system` - Rating system CRUD
- `/rating` - Rating operations
- `/musicbrainz` - MusicBrainz metadata lookups

## Environment Variables

### Frontend (`pulsarr_web/.env`)
```
VITE_API_URL=http://localhost:4004
```

### Backend (`pulsarr_fulcrum/.env`)
```
ENV_NAME=local
POSTGRES_URL=postgres@localhost/pulsarrdb
RUST_PORT=4004
```

## Key Patterns

- **API Key Authentication**: All API calls require `pulsarr-api-key` header, handled automatically by DataRequestHandler
- **Type Synchronization**: Backend OpenAPI spec → frontend types via `npm run generateModels`
- **Database Migrations**: Located in `db/migrations/`, auto-run on backend startup
- **Component Composition**: Modal components use slot-based patterns for flexible content
