# Adventure time API

## Get running locally

### Windows

1. Download rustup init and install [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Run `cargo run`
3. Go to localhost:3000 and be amazed :wink:
4. Get yourself a coffee :coffee:

## Structure

```
src/
├── main.rs              # Entry point (starts the app)
├── routes/              # All route handlers
│   ├── mod.rs
│   ├── hello.rs
│   └── users.rs
├── handlers/            # Business logic (optional if it grows)
│   └── mod.rs
├── models/              # Data models (e.g. request/response structs)
│   └── mod.rs
├── utils/               # Utility functions, extractors, error handling, etc.
│   └── mod.rs
└── state.rs             # App shared state (e.g. DB connection)
```
