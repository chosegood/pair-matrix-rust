FROM rust:1.76

WORKDIR /usr/src/pair-matrix-rust
COPY . .

RUN cargo install --path .

CMD ["pair-matrix-rust"]

