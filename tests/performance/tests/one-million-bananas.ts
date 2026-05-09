import ws, { MessageEventHandler, Socket } from "k6/ws"
import http from "k6/http"
import { check } from "k6"
import { Options } from "k6/options";

export const options: Options = {
  vus: 1,
  iterations: 1,
};

const ws_url = "ws://localhost:3000/ws/subscribe/redhot";
const params = { tags: {} };

const msg_url = "http://127.0.0.1:3000/notify/chilipepper";
const headers = {
  headers: {
    'x-user': "redhot"
  }
};

export default function () {
  let messages: string[] = [];

  ws.connect(ws_url, params, function (socket) {
    socket.on("message", (event) => {
      console.error("received event")
      messages.push(event);
      socket.close();
    });

    http.post(msg_url, null, headers);
  });

  check(messages, {"Received exactly (1) message": () => messages.length === 1});
};
