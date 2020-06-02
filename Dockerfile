FROM debian:buster-slim
WORKDIR /app
RUN useradd -m pexp
USER pexp
COPY --from=ncrmro/rust:sqlx /usr/local/bin/sqlx /usr/local/bin/sqlx
COPY ./config /config
COPY ./migrations /migrations
COPY ./target/release/manage /usr/local/bin/manage
COPY ./target/release/planet-express /usr/local/bin/planet-express
CMD ["planet-express"]
