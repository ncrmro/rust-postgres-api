# Planet-Express
dc is aliased to `docker-compose`

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

`heroku container:release web`
