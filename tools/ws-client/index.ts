const WS_URL = "ws://localhost:3000/ws/subscribe";
const BASE_NOTIFY_URL = "http://localhost:3000/notify";

console.log("🛠️  Interactive Hub Client Started");

const username = prompt("✍️ Enter a username:");

console.log(`🔗 Connecting to ${WS_URL}...`);

const socket = new WebSocket(WS_URL + "/" + username);

socket.onopen = () => {
  console.log("✅ Connected! You can now send messages.");
  console.log("Type 'exit' or 'quit' to stop.\n");
  startInputLoop();
};

socket.onmessage = (event) => {
  console.log(`\n📥 [Broadcast Received]: ${event.data}`);
};

socket.onclose = () => {
  console.log("\n🔌 Server closed the connection. Exiting...");
  process.exit(0);
};

socket.onerror = (error) => {
  console.error("❌ WebSocket Error:", error);
};

async function startInputLoop() {
  while (true) {
    const input = prompt("✍️  Enter message to broadcast:");

    if (!input || input.toLowerCase() === "exit" || input.toLowerCase() === "quit") {
      console.log("👋 Closing connection...");
      socket.close();
      break;
    }

    try {
      const response = await fetch(`${BASE_NOTIFY_URL}/${encodeURIComponent(input)}`);
      if (!response.ok) {
        console.error(`⚠️ Failed to notify: ${response.statusText}`);
      }
    } catch (err) {
      console.error("❌ Fetch error:", err);
    }
  }
}
