##
##  Copyright 2021 neko.rs contributors <https://neko.rs>
##
##  Licensed with GNU Affero General Public License v3.0 or later
##

# Define global arguments
ARG PROJECT="nekobot"
ARG UID="1000"
ARG GID="1000"

####################

FROM rust:slim-bullseye as build

# Update and install system build dependencies
RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get -y upgrade && \
    DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends libssl-dev pkg-config && \
    DEBIAN_FRONTEND=noninteractive apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Create an empty rust project
ARG PROJECT
RUN USER=root cargo new --bin ${PROJECT}
WORKDIR /${PROJECT}

# Precompile project dependencies
COPY Cargo.toml Cargo.toml
RUN cargo build --release

# Remove precompile garbage
RUN rm src/* target/release/deps/${PROJECT}*

# Build the actual project
COPY src src
RUN cargo build --release

####################

FROM debian:bullseye-slim

RUN DEBIAN_FRONTEND=noninteractive apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get -y upgrade && \
    DEBIAN_FRONTEND=noninteractive apt-get -y install --no-install-recommends libssl-dev && \
    DEBIAN_FRONTEND=noninteractive apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set the main executable path
CMD ["./run"]

# Create the main user and group
ARG UID
ARG GID
ARG PROJECT
RUN addgroup --system --gid ${GID} ${PROJECT} && adduser --system --uid ${UID} --gid ${GID} ${PROJECT}

# Create the project directory and switch to the main user
RUN mkdir ${PROJECT} && chown ${PROJECT}:${PROJECT} /${PROJECT}
WORKDIR /${PROJECT}
USER ${PROJECT}

# Copy the built binary from the previous stage
COPY --from=build --chown=${PROJECT}:${PROJECT} /${PROJECT}/target/release/${PROJECT} ./run
