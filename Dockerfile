# syntax=docker/dockerfile:1
FROM rust:1.100.0-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --locked --bin iaop-eligibility-service

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
RUN useradd --system --uid 10001 --create-home appuser
WORKDIR /app
COPY --from=builder /app/target/release/iaop-eligibility-service /app/iaop-eligibility-service
USER appuser
EXPOSE 8080
ENV RUST_LOG=info
HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
    CMD curl -fsS http://127.0.0.1:8080/health/live > /dev/null || exit 1
ENTRYPOINT ["/app/iaop-eligibility-service"]
