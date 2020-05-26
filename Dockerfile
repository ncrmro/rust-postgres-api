FROM rust:1.43 as watcher

WORKDIR /app
EXPOSE 8000

RUN cargo install cargo-watch systemfd

CMD ["systemfd", "--no-pid", "-s", "http::0.0.0.0:8000", "--", "cargo", "watch", "-x", "run"]
