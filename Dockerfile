FROM rust:latest

WORKDIR /usr/src/morse
COPY . .

RUN cargo install --path .

CMD ["morse"]