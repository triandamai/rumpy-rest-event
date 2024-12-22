FROM rustlang/rust:nightly-slim AS builder
RUN apt update  \
    && apt install -y pkg-config \
    && apt install -y libssl-dev \
    && apt install -y libclang-dev \
WORKDIR /app

COPY Cargo.toml .
#COPY Cargo.lock .
COPY src ./src
COPY uploads ./uploads
COPY locales ./locales


RUN cargo +nightly build --release


FROM debian:stable-slim
RUN apt update  \
    && apt install -y pkg-config \
    && apt install -y libssl-dev \
    && apt install -y openssl ca-certificates \
    && apt install -y libclang-dev \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/strong-teams /usr/local/bin
COPY --from=builder /app/locales/ /usr/local/bin/locales
COPY --from=builder /app/uploads/ /usr/local/bin/uploads

ENTRYPOINT ["/usr/local/bin/strong-teams"]