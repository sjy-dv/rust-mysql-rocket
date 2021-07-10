FROM rust:1.52.1

WORKDIR /app

COPY . .

RUN rustup default nightly
RUN cargo build

CMD ["cargo", "run"]