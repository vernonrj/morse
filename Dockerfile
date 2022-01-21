FROM --platform=$BUILDPLATFORM rust:latest as sources

ENV USER=root

# Vendor dependencies for use later
WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
RUN mkdir -p /code/.cargo && cargo vendor > /code/.cargo/config


FROM rust:latest as builder

ENV USER=root

WORKDIR /code

# Cache dependencies
RUN mkdir -p /code/src && touch /code/src/lib.rs
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY --from=sources /code/.cargo /code/.cargo
COPY --from=sources /code/vendor /code/vendor
RUN cargo build --release --offline

# Copy sources in
COPY src/ /code/src/
# And run the build
RUN cargo build --release --offline

# finally package it up
FROM debian:buster-slim
COPY --from=builder /code/target/release/morse  /usr/bin/morse

EXPOSE 80

CMD ["/usr/bin/morse"]