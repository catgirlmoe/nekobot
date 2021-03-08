FROM rust:alpine as builder

RUN apk add --no-cache musl-dev

RUN USER=root cargo new --bin rustapp
WORKDIR ./rustapp
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/rustapp*
RUN cargo build --release


FROM alpine
ARG APP=/usr/src/app


EXPOSE 8080

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rustapp/target/release/rustapp ${APP}/rustapp

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
