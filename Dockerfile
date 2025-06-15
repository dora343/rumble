# build stage
FROM rust:1.87.0-bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/rumble .
COPY --from=builder /app/.env .


CMD [ "./rumble" ]
