# 📦 Dynamic Forex-Like Pricing Vending Machine – Rust Implementation

This project demonstrates a **smart vending machine simulation** built entirely in **Rust**, featuring:

- 🔥 **Market-driven pricing** (like forex & crypto)
- 🔐 **End-to-End Encryption (E2EE)** using X25519 + AES-GCM
- ⚙️ **Self-contained dynamic pricing engine**
- 🧠 **Demand tracking**
- 📉 **Automatic price normalization**
- 🦀 A *single-file* Rust implementation (`main.rs`)

This README explains **EVERYTHING** end-to-end.

---

# 🚀 Project Overview

This vending machine behaves like a **miniature financial market**:

| If this happens | Then price |
|----------------|-----------|
| Many customers buy a product | 🔼 Goes UP |
| Few customers buy | 🔽 Goes DOWN |
| No activity | ➡️ Normalizes toward base price |

Demand influences volatility, and volatility influences price changes.

This makes every vending machine behave like a **micro forex economy**.

---

# 📊 Dynamic Pricing Architecture

Each product stores:

```rust
struct Product {
    name: String,
    base_price: f64,
    current_price: f64,
    demand: f64,        // increases with purchases
    volatility: f64,    // how fast price changes
    min_price: f64,
    max_price: f64,
}
