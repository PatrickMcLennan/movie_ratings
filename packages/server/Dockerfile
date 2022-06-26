FROM rust:1.61.0

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres
