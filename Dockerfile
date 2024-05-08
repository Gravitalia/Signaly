FROM rust:alpine AS builder

RUN USER=root cargo new --bin signaly
WORKDIR /signaly

ENV     RUSTFLAGS="-C target-feature=-crt-static"
RUN     apk add -q --update-cache --no-cache build-base openssl-dev musl pkgconfig protobuf-dev

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./signaly ./signaly
COPY ./signaly-db ./signaly-db
COPY ./signaly-error ./signaly-error
COPY ./signaly-telemetry ./signaly-telemetry

RUN cargo build --release

FROM alpine:3 AS runtime

RUN apk add --no-cache libgcc

RUN addgroup -S appgroup && adduser -S rust -G appgroup
USER rust

COPY --from=builder /signaly/target/release/signaly /bin/signaly

CMD ["./bin/signaly"]
