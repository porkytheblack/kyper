FROM rust:latest

WORKDIR /usr/src/kyper

COPY . .

RUN cargo build --release

CMD cargo run --release