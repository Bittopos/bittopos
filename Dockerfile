FROM rust:1-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p bittopos-server

FROM debian:bookworm-slim AS runtime
RUN useradd --system --uid 10001 --create-home bittopos \
    && mkdir -p /var/lib/bittopos \
    && chown -R bittopos:bittopos /var/lib/bittopos
COPY --from=builder /app/target/release/bittopos-server /usr/local/bin/bittopos-server
USER bittopos
ENV BIND_ADDR=0.0.0.0:3000
ENV BITTOPOS_DATA_DIR=/var/lib/bittopos
EXPOSE 3000
VOLUME ["/var/lib/bittopos"]
ENTRYPOINT ["/usr/local/bin/bittopos-server"]
