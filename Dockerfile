ARG RUST_IMAGE=rust:1.43
ARG BUILT_IMAGE=debian:buster-slim
ARG SQLX_IMAGE=sqlx

###
# Development
FROM ${RUST_IMAGE} as sqlx
RUN cargo install --git https://github.com/launchbadge/sqlx.git --rev a9fb19b37da0e77fd891b8a2358733c563115a5c cargo-sqlx

FROM ${RUST_IMAGE} as watcher_build
RUN cargo install cargo-watch systemfd

FROM ${RUST_IMAGE} as watcher
WORKDIR /app
RUN useradd -m pexp
USER pexp
COPY --from=watcher_build /usr/local/cargo/bin/systemfd /usr/local/bin/systemfd
COPY --from=watcher_build /usr/local/cargo/bin/cargo-watch /usr/local/bin/cargo-watch
COPY --from=sqlx /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
CMD ["systemfd", "--no-pid", "-s", "http::0.0.0.0:8000", "--", "cargo", "watch", "--watch", "src", "--exec", "run"]

###
# Production
FROM ${RUST_IMAGE} as builder
WORKDIR /app
COPY . /app
ENV DATABASE_URL=postgres://pexp:pexp@localhost:5432/pexp
RUN cargo install --path .

FROM ${BUILT_IMAGE} as built
WORKDIR /app
RUN useradd -m pexp
USER pexp
COPY --from=${SQLX_IMAGE} /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY ./config /config
COPY ./migrations /migrations
COPY --from=builder /usr/local/cargo/bin/planet-express /usr/local/bin/planet-express
CMD ["planet-express"]
