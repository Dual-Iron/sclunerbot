FROM rust:1.60

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ./target/release/sclunerbot
