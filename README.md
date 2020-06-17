# Planet-Express

A REST API boilerplate written in Rust, heavily inspired by [Django](https://www.djangoproject.com).

## TODO

- [ ] Testing - Unit
- [x] Testing - Integration
- [ ] How to implement custom users with `core::auth`
- [x] Develop locally or inside a docker container.
- [x] Heroku Deployed Docker Container

`d` and `dc` have been aliased to `docker` and `docker-compose`

## Database Operations
Creating a new migration
`dc exec app sqlx mig add user`

Running migrations
`dc exec app sqlx mig run`

Iterating on Schema
```
dc down && \
docker volume rm planet-express_pgdata && \
dc up migrations && \
dc up tests
```

## Testing

`dc exec app cargo test`


## Build & Release
Using Docker containers here means we can transition from Heroku in the future easily
This should all ideally happen in CI.
`heroku container:login`

Due to SQLx requiring a database during compile time to check our SQL statements
we need to set our network to host and have a database live with our migrations.
`dc up -d db app-migrations && d build --network="host" -t web .`

`d tag web registry.heroku.com/planetexpres/web`
`d push registry.heroku.com/planetexpres/web`

If you haven't added a database yet run
`heroku addons:create heroku-postgresql:hobby-dev`

`heroku config:set APP_AUTH__JWT_SECRET=supersecret`
`heroku config:set APP_AUTH__PASSWORD_SALT=supersecret`

Finally release
`heroku container:release web`
