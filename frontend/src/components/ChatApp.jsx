import React, { useEffect, useState, useRef } from "react";

export default function ChatApp() {
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState("");
  const socketRef = useRef(null);

  useEffect(() => {
    const socket = new WebSocket("ws://localhost:8080/ws");
    socketRef.current = socket;

    socket.onopen = () => console.log("✅ Connected to Rust backend");
    socket.onmessage = (event) =>
      setMessages((prev) => [...prev, event.data]);
    socket.onclose = () => console.log("❌ Disconnected");
    socket.onerror = (err) => console.error("⚠️ Error:", err);

    return () => socket.close();
  }, []);

  const sendMessage = () => {
    if (input.trim() && socketRef.current?.readyState === WebSocket.OPEN) {
      socketRef.current.send(input);
      setInput("");
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-4">
      <div className="w-full max-w-md bg-white shadow-lg rounded-2xl p-5">
        <h2 className="text-2xl font-bold mb-4 text-indigo-600 text-center">
          💬 Rust + React Chat
        </h2>

        <div className="h-96 overflow-y-auto border border-gray-200 rounded-lg p-3 bg-gray-50">
          {messages.map((msg, i) => (
            <div
              key={i}
              className="bg-indigo-100 text-gray-800 px-3 py-1 rounded-lg mb-2 text-sm"
            >
              {msg}
            </div>
          ))}
        </div>

        <div className="flex mt-4 gap-2">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && sendMessage()}
            placeholder="Type a message..."
            className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring focus:ring-indigo-200"
          />
          <button
            onClick={sendMessage}
            className="bg-indigo-600 text-white px-4 py-2 rounded-lg hover:bg-indigo-700 transition"
          >
            Send
          </button>
        </div>
      </div>
    </div>
  );
}
