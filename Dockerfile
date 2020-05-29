FROM rust:1.43 as base
WORKDIR /app
EXPOSE 8000
RUN cargo install --git https://github.com/launchbadge/sqlx.git --rev a9fb19b37da0e77fd891b8a2358733c563115a5c cargo-sqlx

FROM base as watcher
RUN cargo install cargo-watch systemfd
CMD ["systemfd", "--no-pid", "-s", "http::0.0.0.0:8000", "--", "cargo", "watch", "--watch", "src", "--exec", "run"]

FROM base as builder
COPY . /app
ENV DATABASE_URL=postgres://pexp:pexp@localhost:5432/pexp
RUN cargo install --path .

FROM debian:buster-slim
RUN useradd -m pexp
USER pexp
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY ./config /config
COPY ./migrations /migrations
COPY --from=builder /usr/local/cargo/bin/planet-express /usr/local/bin/planet-express
CMD ["planet-express"]
