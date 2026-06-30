# Weather CLI 🌤

A command-line weather checking program written in Rust

## 🚀 Getting Started

### 1. Install Rust (if you don't have it)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Clone and Run
```bash
git clone https://github.com/nattapong18-en/weather.git
cd weather
cargo run -- Bangkok
```

## 📖 Usage

```bash
# Pass city name after --
cargo run -- Bangkok
cargo run -- "New York"
cargo run -- London
cargo run -- Tokyo
```

### Example Output
```bash
$ cargo run -- Bangkok
🌤 Clear
🌡 32°C
💧 65%
```

## 🛠️ Install as Global Command (optional)

```bash
cargo install --path .
weather Bangkok
```

## ⚠️ Important

**Always provide a city name!**

```bash
# ❌ Wrong
cargo run

# ✅ Correct
cargo run -- Bangkok
```

## 📦 Dependencies

- `reqwest` - HTTP Client
- `serde` - JSON Parser
- `tokio` - Async Runtime

---

**Made with ❤️ in Rust** 🦀
```

