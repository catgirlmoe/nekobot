##
##  Copyright 2021 catgirl.moe contributors <https://catgirl.moe>
##
##  Licensed with GNU Affero General Public License v3.0 or later
##

FROM rust:slim

# Create new project
RUN USER=root cargo new --bin nekobot

# Create user for nekobot and chown directory
RUN addgroup --system --gid 1000 nekobot && adduser --system --uid 1000 --gid 1000 nekobot
RUN chown -R nekobot:nekobot /nekobot

# Switch to the project directory and user
WORKDIR /nekobot
USER nekobot

# Expose port and set entrypoint
EXPOSE 8080
CMD ["./target/release/nekobot"]

# Precompile dependencies
COPY --chown=nekobot:nekobot Cargo.toml Cargo.toml
RUN cargo build --release

# Remove precompile garbage
RUN rm src/*.rs
RUN rm target/release/deps/nekobot*

# Add the source code and build the final project
COPY --chown=nekobot:nekobot src src
RUN cargo build --release
