
# 💬 Chat App (Rust + React + Tailwind)

A **real-time chat application** built with a **Rust backend (WebSocket)** and **React + Tailwind frontend**.  
This project demonstrates **modern full-stack development** with **performance**, **scalability**, and **real-time communication** using WebSockets.



## 🚀 Project Structure

```

chat-app/
│
├── backend/           # Rust WebSocket Server
│   ├── src/
│   ├── Cargo.toml
│   └── README.md
│
└── frontend/          # React + Tailwind Web App
├── src/
├── package.json
└── README.md

````

---

## ⚙️ Backend (Rust)

### 🧠 Tech Stack
- **Rust**
- **Tokio** – for async runtime  
- **Axum / Warp** – for WebSocket handling  
- **Serde** – for JSON serialization  
- **Futures** – for async stream management  

### 🧩 Features
- Real-time WebSocket connection
- Multiple user message broadcasting
- Lightweight and fast async server
- Connection management and graceful shutdown

### 🔧 Setup Instructions

#### 1️⃣ Navigate to backend
```bash
cd backend
````

#### 2️⃣ Install dependencies

```bash
cargo build
```

#### 3️⃣ Run the server

```bash
cargo run
```

By default, the WebSocket server runs on:

```
ws://localhost:8080/ws
```

---

## 🎨 Frontend (React + Tailwind)

### 🧠 Tech Stack

* **React (Vite or CRA)**
* **Tailwind CSS** – for fast styling
* **WebSocket API** – for real-time chat

### ✨ Features

* Connects to the Rust WebSocket server
* Sends and receives messages instantly
* Modern responsive UI
* Displays chat messages in real time

### 🔧 Setup Instructions

#### 1️⃣ Navigate to frontend

```bash
cd frontend
```

#### 2️⃣ Install dependencies

```bash
npm install
```

#### 3️⃣ Start the React app

```bash
npm start
```

The frontend will run on:

```
http://localhost:3000
```

> Make sure your Rust backend is running before starting the frontend.

---

## 🔌 How It Works

1. **User connects** from the frontend → WebSocket opens at `ws://localhost:8080/ws`.
2. **Frontend sends a message** using `socket.send("Hello")`.
3. **Backend receives** the message and **broadcasts** it to all connected clients.
4. **Each client** receives the broadcast and displays it instantly.

---

## 🧱 Example Code Snippets

### Frontend WebSocket Connection

```js
const socket = new WebSocket("ws://localhost:8080/ws");

socket.onopen = () => {
  console.log("✅ Connected to server");
  socket.send("Hello from browser 👋");
};

socket.onmessage = (e) => {
  console.log("📩 Message:", e.data);
};

socket.onclose = () => console.log("❌ Disconnected");
```

### Backend (Rust) WebSocket Handler

```rust
async fn handle_socket(ws: WebSocketUpgrade, state: Arc<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| client_connection(socket, state))
}
```

---

## 🧪 Testing

Open **two browser tabs** and connect both to:

```
http://localhost:3000
```

Type messages in one tab — you’ll see them appear **in real-time** in the other tab!

---

## 🗂️ Environment Details

| Component        | Port | Description      |
| ---------------- | ---- | ---------------- |
| Frontend (React) | 3000 | Chat UI          |
| Backend (Rust)   | 8080 | WebSocket Server |

---

## 📦 Deployment Notes

When pushing to GitHub:

* ✅ **Include** `src/`, `Cargo.toml`, `package.json`, and `tailwind.config.js`
* ❌ **Exclude** `target/` and `node_modules/` folders
  Add this in `.gitignore`:

```gitignore
# Rust
/target

# Node
/node_modules
```

---

## 🧑‍💻 Author

**Sivaji Gadidala**
Full Stack Developer | Rust | React | Blockchain
🔗 [GitHub](https://github.com/yourusername)

---
