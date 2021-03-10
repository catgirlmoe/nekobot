FROM rust:alpine as builder
# Install required dependencies
RUN apk add --no-cache musl-dev
# Create new project
RUN USER=root cargo new --bin moebot
WORKDIR ./moebot
# Copy the Package file
COPY ./Cargo.toml ./Cargo.toml
# Install cargo dependencies
RUN cargo build --release
# Strip possible garbage
RUN rm src/*.rs
# Remove existing builds
RUN rm ./target/release/deps/moebot*
# Add the sources
ADD src/* ./src/
# Build the final project
RUN cargo build --release

FROM alpine
ARG APP=/usr/src/app

EXPOSE 8080

COPY --from=builder /moebot/target/release/moebot ${APP}/moebot

WORKDIR ${APP}


CMD ["./moebot"]
