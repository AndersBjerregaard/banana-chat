# tests/api

To install dependencies:

```bash
bun install
```

To run:

```bash
bun run index.ts
```

This project was created using `bun init` in bun v1.3.11. [Bun](https://bun.com) is a fast all-in-one JavaScript runtime.

## Test

This is an integration test project for the websocket logic of the api in `src/api/`.

To run the test, first start the api:

```bash
# In the directory src/api/
cargo run
```

Run the test in this directory:

```bash
bun run index.ts
```

Example output:
```markdown
🚀 Starting Integration Test...
✅ WebSocket Connected!
📡 Triggering a server broadcast...
📩 Received Message: "HelloFromBun"
✨ Test Passed: Message received successfully!
🔌 Connection closed.
```
