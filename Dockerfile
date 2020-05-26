FROM rust:1.43 as watcher

WORKDIR /app
EXPOSE 8000

RUN cargo install cargo-watch diesel_cli

CMD ["cargo", "watch", "-x", "run"]
