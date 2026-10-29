FROM rust:1.99-slim-trixie AS builder

WORKDIR /app

# Build dependencies in their own layer so source changes don't rebuild them
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release --locked && rm -r src

COPY src src
RUN touch src/main.rs && cargo build --release --locked

FROM gcr.io/distroless/cc-debian13:nonroot

COPY --from=builder /app/target/release/tristan /usr/local/bin/tristan

ENV RUST_LOG="info"
# teloxide shuts down gracefully on SIGINT only
STOPSIGNAL SIGINT

ENTRYPOINT ["tristan"]
