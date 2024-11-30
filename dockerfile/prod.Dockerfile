FROM rustlang/rust:nightly-slim AS builder
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src

RUN cargo +nightly build --release

FROM debian:stable-slim
RUN apt update \
    && apt install -y libssl-dev \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
COPY --from=builder /app/target/release/strong-teams /usr/local/bin

EXPOSE 7001
ENTRYPOINT ["/usr/local/bin/strong-teams"]