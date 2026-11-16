FROM rust:1-bookworm AS builder
WORKDIR /workspace
COPY . .
RUN cargo build --release -p guardian-api

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home app
WORKDIR /app
COPY --from=builder /workspace/target/release/guardian-api /usr/local/bin/guardian-api
USER 10001:10001
EXPOSE 8080
ENTRYPOINT ["guardian-api"]
