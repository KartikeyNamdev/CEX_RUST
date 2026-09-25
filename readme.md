# CEX — Rust Server

A centralized exchange (CEX) backend, built in Rust with Actix-web, as a learning + portfolio project. This repo is the **Rust implementation**; a parallel TypeScript implementation of the same system exists separately, built to compare how the two languages/runtimes handle the same problem.

## What this is

A CEX lets users deposit fiat (INR/USD) and crypto assets, then place buy/sell orders against each other through an order book maintained by the exchange — as opposed to a DEX, where swaps settle on-chain against a liquidity pool with no custodian in between. Here, the exchange (this server) custodies balances and matches orders itself.

## Current status

This is an early, in-progress build. Right now the server handles **user accounts and balance lookup only** — the order book and matching engine are not implemented yet (see [Roadmap](#roadmap)).

State is currently **in-memory only** (no database) — all users and balances reset when the server restarts. This is intentional at this stage, to focus on getting the concurrency model and Actix-web patterns right before adding persistence.

## Tech stack

- **Rust** + **Actix-web** — HTTP server framework. Actix runs each incoming request on a Tokio task, so the server is multithreaded by default (unlike Node.js, which is single-threaded).
- **Serde** (`serde`, `serde_json`) — JSON serialization/deserialization for request bodies and responses.
- **`std::sync::Mutex`** — guards shared state (the user list, the user-index counter) so concurrent requests can't corrupt it. Wrapped in `web::Data`, which internally uses an `Arc` to share that state cheaply across every worker thread.

## Architecture

### Shared state

```rust
struct AppState {
    users: Mutex<Vec<USER>>,
    user_index: Mutex<u32>,
}
```

`web::Data<AppState>` is handed to every request handler. Cloning it (`app_state.clone()`) only bumps an `Arc` reference count — it's cheap, not a deep copy. Each handler locks only the specific `Mutex` field it needs, for as short a time as possible, to avoid blocking other requests.

### Endpoints (implemented)

| Method | Path              | Description                                                                                                                                              |
| ------ | ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `POST` | `/signup`         | Creates a new user. Rejects if the username already exists.                                                                                              |
| `POST` | `/login`          | Validates username/password, returns the user's public info.                                                                                             |
| `GET`  | `/balance/usd`    | Returns a user's USD balance. Currently identified via a query parameter (`?user_index=`) — not yet backed by real auth (see Roadmap).                   |
| `GET`  | `/balance?asset=` | Returns a user's balance for a specific asset (`sol`, `btc`, or `eth`). Identified the same way as `/balance/usd`, via `user_index` in the query string. |

### Data model (`structs.rs`)

- `USER` — internal record: index, username, password, `usd_balance`, and an `Assets` struct (`sol_balance`, `btc_balance`, `eth_balance`).
- `PublicUser` — the subset of a `USER` that's safe to return in API responses (no password).
- `SignupBody` — incoming JSON for `/signup` and `/login`.
- `SignUpResponse` — response shape for `/signup` and `/login`.
- `USDBalanceResponse` — response shape for `/balance/usd`.
- `AssetBalanceResponse` — response shape for `/balance?asset=`.
- `UserQuery` — query-string extractor shared by `/balance/usd` and `/balance?asset=`.

## Running locally

```bash
cargo run
```

Server listens on `127.0.0.1:3001`.

### Example requests

```bash
# Signup
curl -X POST http://127.0.0.1:3001/signup \
  -H "Content-Type: application/json" \
  -d '{"username": "kartikey", "password": "secret"}'

# Login
curl -X POST http://127.0.0.1:3001/login \
  -H "Content-Type: application/json" \
  -d '{"username": "kartikey", "password": "secret"}'

# Check USD balance
curl "http://127.0.0.1:3001/balance/usd?user_index=1"

# Check a specific asset balance
curl "http://127.0.0.1:3001/balance?asset=sol&user_index=1"
```

## Roadmap

Planned, not yet built:

- [ ] `POST /balance/onramp` — simulate fiat deposit
- [ ] `POST /balance/deposit` — simulate crypto asset deposit
- [ ] `POST /order` — place a market or limit order
- [ ] In-memory order book with price-time priority matching (FIFO within a price level)
- [ ] Balance locking when an order is placed, unlocking on cancel/fill
- [ ] Real authentication (token-based — a client should _prove_ who it is via a token from `/login`, not just claim a `user_index` in a query param)
- [ ] Persistence (currently all state is lost on restart)
- [ ] WebSocket feed for live order book / trade updates

## Notes on concurrency

Since Actix-web dispatches requests across multiple threads, any state shared between requests must be protected against concurrent mutation:

- `Arc` (via `web::Data`) lets multiple threads hold a reference to the _same_ underlying state safely.
- `Mutex` ensures only one thread can read/write the guarded data at a time.
- Locks are scoped as narrowly as possible in each handler to avoid holding a lock longer than necessary, and to avoid deadlocking by trying to lock the same `Mutex` twice within one request.

## Companion project

A TypeScript/Node.js implementation of the same CEX lives alongside this one, built in parallel to compare Rust's ownership/concurrency model against Node's single-threaded, async-by-default approach.
