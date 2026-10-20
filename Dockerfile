FROM rust:1-bookworm AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends libsqlite3-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release --locked -p bittopos-server

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home bittopos \
    && mkdir -p /var/lib/bittopos \
    && chown -R bittopos:bittopos /var/lib/bittopos
COPY --from=builder /app/target/release/bittopos-server /usr/local/bin/bittopos-server
USER bittopos
ENV BIND_ADDR=0.0.0.0:3000
ENV BITTOPOS_DB_PATH=/var/lib/bittopos/bittopos.sqlite3
ENV SESSION_COOKIE_SECURE=true
EXPOSE 3000
VOLUME ["/var/lib/bittopos"]
ENTRYPOINT ["/usr/local/bin/bittopos-server"]
