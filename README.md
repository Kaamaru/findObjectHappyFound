
# HappyFound Rust Backend
This is a Rust backend built with [Axum](https://crates.io/crates/axum)
## Prequisites
- [Rust Programming Language](https://rust-lang.org/)


## Installation

### **Clone the Repository in the branch**

```bash
git clone --branch rust-api https://github.com/Hatyaiwittayalai/FindObjectHappyFound.git --single-branch
cd FindObjectHappyFound
````
---

### **Run with Cargo**

```bash
# Build the project
cargo build

# Run the server
cargo run
```
---

* By default, the server runs on `http://localhost:3000`.
* Set any required environment variables in a `.env` file:

```env
# /.env
VARIABLE1=...
VARIABLE2=...
...
```
```docker
### **Using with Docker**

This is the method I found online.
Please utilize your own experiences and knowledges to proceed.

# Stage 1: Build the Rust application
FROM rust:1.74-slim-buster AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Create the final, minimal image
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/your_app_name .
CMD ["./your_app_name"]
```
