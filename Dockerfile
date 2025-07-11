FROM rust:1.77

WORKDIR /app

# Pre-copy for caching dependencies
COPY Cargo.toml .
# COPY Cargo.lock .

# Avoid build fail due to missing src
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy actual source
COPY . .

RUN cargo build --release

CMD ["./target/release/rust-todo-cli"]
