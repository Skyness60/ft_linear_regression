FROM rust:1.88

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--bin", "train"]
