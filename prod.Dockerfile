FROM rustlang/rust:nightly-slim AS builder
RUN apt update \
    && apt install -y pkg-config \
    && apt install -y libssl-dev
WORKDIR /app

COPY Cargo.toml .
#COPY Cargo.lock .
COPY src ./src
COPY uploads ./uploads
COPY locales ./locales


RUN cargo +nightly build --release

FROM debian:stable-slim
RUN apt update \
    && apt install -y pkg-config \
    && apt install -y libssl-dev \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*
COPY --from=builder /app/target/release/strong-teams /usr/local/bin

ENTRYPOINT ["/usr/local/bin/strong-teams"]