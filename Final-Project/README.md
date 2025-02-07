# Final Project: Full Sui Blockchain dApp

## 📌 Overview
This final project integrates all the lessons learned in previous modules to build a **complete Sui blockchain dApp** using:
- **Rust for backend API** (Axum-based)
- **Move smart contracts for blockchain logic**
- **Yew for a WebAssembly-based frontend**
- **Secure interactions between frontend and backend**

## 📂 Project Structure
```plaintext
Final-Project/
│── backend/                 # Rust API for Blockchain
│   ├── main.rs              # Axum server with API endpoints
│   ├── Cargo.toml           # Rust dependencies
│── move_contract/           # Move Smart Contract
│   ├── sources/contract.move  # Move contract logic
│   ├── build/               # Compiled contract files
│── frontend/                # Yew-based WebAssembly frontend
│   ├── src/main.rs          # Yew application entry point
│   ├── Cargo.toml           # Rust dependencies for frontend
│── README.md                # Project documentation
```

## 🔧 Setup Instructions
### 1️⃣ **Clone the Repository**
```sh
git clone https://github.com/DevCorpOrg/201-jvm.git
cd 201-jvm/Final-Project
```

### 2️⃣ **Set Up the Backend API (Rust + Axum)**
```sh
cd backend
cargo run
```
- Runs an Axum-based Rust API on **http://127.0.0.1:8080**
- Exposes `/balance` endpoint to retrieve user balance

### 3️⃣ **Deploy the Move Smart Contract**
```sh
cd move_contract
sui move build
sui move publish
```
- Compiles and deploys the Move contract to Sui blockchain.

### 4️⃣ **Run the Yew Frontend (WASM)**
```sh
cd frontend
trunk serve
```
- Starts a **WASM-based frontend** that interacts with the Rust API and Move contract.
- Open **http://127.0.0.1:8080** in your browser.

## 🚀 Features Implemented
✅ **Query Sui blockchain balance** using Rust API
✅ **Execute Move smart contract transactions**
✅ **Secure API communication between frontend & backend**
✅ **Full-stack decentralized application using Rust + WASM**

## 📌 Next Steps
- **Deploy to AWS or Cloudflare Pages**
- **Add user authentication (JWT or OAuth)**
- **Expand Move smart contract functionality**

## 🛠️ Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature-xyz`)
3. Commit your changes (`git commit -m "Added feature XYZ"`)
4. Push to your branch (`git push origin feature-xyz`)
5. Open a Pull Request

---

🚀 **Now, run your full Sui blockchain dApp!** 🚀
