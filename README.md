# Planet-Express

## TODO

- [ ] Testing - Unit
- [x] Testing - Integration
- [x] Docker Build
- [x] Heroku Deployed Docker Container
- [ ] Coverage
- [ ] Sentry

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

## Images
We provide a few public images due to how like cargo-watcher and

### SQLx 
Used by watcher and the final docker image, contains SQLx CLI binary.
```bash
docker build \
--target sqlx \
--tag ncrmro/rust:sqlx -f Dockerfile.rust .
```

`docker push ncrmro/rust:sqlx`

### Watcher
Has cargo-watch and sqlx binaries available.
`docker build --target watcher --tag ncrmro/rust:watcher -f Dockerfile.rust .`

`docker push ncrmro/rust:watcher`

### Built Image
To save time let the precached compose container build the image and
copy binary directly from volume.

```bash
docker build --target built
```
