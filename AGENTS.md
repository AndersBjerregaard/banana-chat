# AGENTS.md

## Project Structure

**banana-chat** is a broadcast WebSocket API server in Rust + JavaScript clients.

- **src/api/**: Rust backend (Axum + Tokio). Single-package app, no workspace.
- **tests/api/**: Bun-based integration test. Must run after API starts.
- **tools/ws-client/**: Interactive Bun CLI client for manual testing.

## Build & Run Commands

### API (Rust)
```bash
cd src/api
cargo run              # Starts server on 0.0.0.0:3000
cargo build --release # Production build
```

### Tests
```bash
# Terminal 1: Start the API
cd src/api && cargo run

# Terminal 2: Run integration test (requires API running)
cd tests/api
bun install            # One-time setup
bun run index.ts       # Runs test, expects API on ws://localhost:3000/ws/subscribe
```

### Client Tool
```bash
cd tools/ws-client
bun install
bun run index.ts       # Interactive prompt to broadcast messages
```

## Key Implementation Details

### Message Flow
1. **HTTP POST**: `GET /notify/{msg}` broadcasts via `state.hub.tx.send(msg)`
2. **WebSocket RX**: `GET /ws/subscribe` creates broadcast receiver, forwards incoming messages
3. **Broadcast Channel**: Fixed capacity of **16 messages** (src/api/src/main.rs:11)
   - Slow receivers will lag if they fall behind by >16 messages
   - On lag, receiver gets `RecvError::Lagged(n)` (see src/api/README.md for details)

### Architecture
- **state.rs**: `AppState` wraps `AppHub` (holds `broadcast::Sender<String>`)
- **routes/**: HTTP and WebSocket routes mounted at `/` and `/ws` respectively
- **handlers/ws_handler.rs**: Splits socket, subscribes to broadcast, echoes incoming messages
- Logging uses `println!` (no structured logging framework)

## Important Constraints

- **Edition**: Cargo.toml specifies `edition = "2024"` (non-standard; likely means "2021" or experimental)
  - If you see edition-related compile errors, check if this is the issue
- **No dependencies on logging/tracing**: Debug via `println!` for now
- **Bun required**: Tests and client tools require `bun` runtime (not Node.js)
- **API hardcoded to port 3000**: Tests and client tools assume this address

## Testing Gotchas

- Integration test **must** have API running first; it will hang if API is not reachable
- Test has 5-second timeout (tests/api/index.ts:38)—if slower, test fails
- WebSocket connection address: `ws://localhost:3000/ws/subscribe` (note `/ws/` prefix)
- Trigger notifications via: `http://localhost:3000/notify/YourMessage` (HTTP GET)

## Common Tasks

**Run API only**: `cd src/api && cargo run`

**Test end-to-end**: Start API in one terminal, then `cd tests/api && bun run index.ts`

**Add a route**: Modify `src/api/src/routes/mod.rs` or add new route file, import in routes/mod.rs

**Change broadcast capacity**: Edit `broadcast::channel(16)` in src/api/src/main.rs

**Fix slow receiver lag**: Increase capacity in main.rs, or handle `RecvError::Lagged` in ws_handler.rs
