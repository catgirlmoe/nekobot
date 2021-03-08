FROM rust:alpine as builder

RUN apk add --no-cache musl-dev

RUN USER=root cargo new --bin moebot
WORKDIR ./moebot
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/moebot*
RUN cargo build --release


FROM alpine
ARG APP=/usr/src/app


EXPOSE 8080

COPY --from=builder /moebot/target/release/moebot ${APP}/moebot

WORKDIR ${APP}
