# Planet-Express

## Database Operations

dc is aliased to `docker-compose`
`dc run app diesel print-schema > src/db/schema.rs`

`dc run app diesel migration generate create_users`

`dc run app diesel migration run`
