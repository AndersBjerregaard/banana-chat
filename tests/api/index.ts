const WS_URL = "ws://localhost:3000/ws/subscribe";
const TRIGGER_URL = "http://localhost:3000/notify/HelloFromBun";

console.log("🚀 Starting Integration Test...");

const socket = new WebSocket(WS_URL);

socket.onopen = async () => {
  console.log("✅ WebSocket Connected!");

  // Once connected, let's trigger a message via the HTTP endpoint
  console.log("📡 Triggering a server broadcast...");
  const response = await fetch(TRIGGER_URL);
  const status = await response.text();
  console.log(`🌐 Server Response: ${status}`);
};

socket.onmessage = (event) => {
  console.log(`📩 Received Message: "${event.data}"`);
  
  if (event.data === "HelloFromBun") {
    console.log("✨ Test Passed: Message received successfully!");
    socket.close();
    process.exit(0);
  }
};

socket.onerror = (error) => {
  console.error("❌ WebSocket Error:", error);
  process.exit(1);
};

socket.onclose = () => {
  console.log("🔌 Connection closed.");
};

// Timeout after 5 seconds if nothing happens
setTimeout(() => {
  console.error("⏰ Test timed out!");
  process.exit(1);
}, 5000);
