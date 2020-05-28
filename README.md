# Planet-Express
dc is aliased to `docker-compose`

## Database Operations
Creating a new migration
`dc exec app sqlx mig add user`

Running migrations
`dc exec app sqlx mig run`

## Testing

`dc exec app cargo test`
