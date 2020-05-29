# Planet-Express

### TODO

- [ ] Testing - Unit
- [x] Testing - Integration
- [x] Docker Build
- [x] Heroku Deployed Docker Container

`d` and `dc` have been aliased to `docker` and `docker-compose`

## Database Operations
Creating a new migration
`dc exec app sqlx mig add user`

Running migrations
`dc exec app sqlx mig run`

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

Finally release
`heroku container:release web`
