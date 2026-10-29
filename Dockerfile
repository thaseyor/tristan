FROM rust:1.99

RUN mkdir -p /home/app
WORKDIR /home/app
COPY . .

RUN cargo build -r
ENV RUST_LOG="info"

ENTRYPOINT cargo run -r
