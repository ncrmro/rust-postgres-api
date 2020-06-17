# Planet-Express

A REST API boilerplate written in Rust, heavily inspired by [Django](https://www.djangoproject.com).

Table of contents
- [Tech Overview](#technology-overview)
- [Getting started](#getting-started)
  - [Docker](#docker)
  - [Locally](#locally)
- [Creating a new migration](#database-migrations)
- [Deployment](#deployment)

## Technology Overview

| Problem            | Solution                                                | Result                                                                                            |
| ------------------ | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Database           | [Postgres](https://www.postgresql.org)                  | It's Postgres :)                                                                                  |
| Rust SQL Toolkit   | [SQLx](https://github.com/launchbadge/sqlx)             | Query the database with compile timed SQL (no ORM!)                                               |
| Webframework       | [Actix Web](https://github.com/actix/actix-web)         | Very fast web server                                                                              |
| OpenAPI Generation | [Paperclip](https://github.com/wafflespeanut/paperclip) | Automatically generated specification of your API, which can be used to generate your API client. |

## TODO

- [ ] Testing - Unit
- [x] Testing - Integration
- [ ] How to implement custom users with `core::auth`
- [x] Develop locally or inside a docker container.
- [x] Heroku Deployed Docker Container

## Getting started

To quickly get up an running you can use Docker Compose. Although your
IDE will typically work better if everything installed locally.

### Docker

Make sure you have [Docker Desktop](https://docs.docker.com/desktop/)

This will start the database and run migrations.

`docker-compose up migrations`

Start your local development server, any changes will reload the code.

`docker-compose up app`

Now browse [localhost:8000](http://localhost:8000)

Lets now start the integration tests watcher.

Stop the app (quicker iteration as we dont need to wait on the app)
`docker-compose stop app`

Same as the app container any code changes will automatically
`docker-compose up tests`

this will start and reload
the integration tests when any code changes. We stop the app container
as the two containers will often

### Locally

Make sure you have [rust and cargo](https://www.rust-lang.org/learn/get-started) and
postgres installed.

Create your database.

`createdb pexp`

And run migrations

`sqlx migrate run`

Install [cargo watch](https://github.com/passcod/cargo-watch)

`cargo install cargo-watch`

This is equivalent to `cargo run` but with autoreload on changes in `src` folder.

`cargo watch --watch src --exec run`

To run tests we can use.

`cargo run tests`

To run a specific test.

`cargo test users::model::test_model_create`

This will run the `test_model_create` in `tests/users`

## Database Migrations

Creating a new migration using docker.
`docker-compose exec app sqlx mig add test_migration`

or locally

`sqlx migrate add test_migration`

Running migrations
`docker-compose exec app sqlx mig run`

If you find yourself quickly iterating on your schema this will reset the database
with your new schema.

```
docker-compose down && \
docker volume rm planet-express_pgdata && \
docker-compose up migrations && \
docker-compose up tests
```

## Deployment

The following has been automated into the [CI pipeline](.github/workflows/main.yml#L67-L90) and here for reference.

Using Docker containers here means we can transition from Heroku in the future easily.

Login

`heroku container:login`

Due to SQLx requiring a database during compile time to check our SQL statements
we need to set our network to host and have a database live with our migrations.

`docker-compose up -d migrations && docker build --network="host" -t web .`

`docker tag web registry.heroku.com/planetexpres/web`
`docker push registry.heroku.com/planetexpres/web`

If you haven't added a database yet run

`heroku addons:create heroku-postgresql:hobby-dev`

Configure our secrets
`heroku config:set APP_AUTH__JWT_SECRET=supersecret`

`heroku config:set APP_AUTH__PASSWORD_SALT=supersecret`

Finally release

`heroku container:release web`
