FROM debian:buster-slim
ENV APP_ENV=production
RUN apt update && apt install ca-certificates libssl1.1 -y && rm -rf /var/lib/apt/lists/*
RUN useradd -m pexp
USER pexp
COPY --from=ncrmro/rust:sqlx /usr/local/bin/sqlx /usr/local/bin/sqlx
COPY ./config /config
COPY ./migrations /migrations
COPY ./target/release/manage /usr/local/bin/manage
COPY ./target/release/planet-express /usr/local/bin/planet-express
CMD ["planet-express"]
