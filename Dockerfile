FROM rust:1.69 as build

RUN USER=root cargo new --bin signaly
WORKDIR /signaly

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release \
 && rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/signaly* \
 && cargo build --release

FROM rust:1.69-slim-buster

COPY --from=build /signaly/target/release/signaly .

EXPOSE 1112
CMD ["./signaly"]
