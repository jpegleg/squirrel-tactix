# squirrel-tactix 🐿️ 

An Actix API microservice that uses Diesel Postgresql.

## credentials, diesel migration, and initialization

Current version has a `.env` file that contains the database credentials. Example `.env` file:

```
RUST_LOG=squirrel_tactix=info,actix=info,diesel_migrations=info
DATABASE_URL=postgres://postgres:mypassword@localhost:5432
HOST=127.0.0.1
PORT=8007
```

Diesel is an ORM and has migrations. We can set up diesel command line to run migrations.

```
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration generate create_users
# create tables and insert initial data etc
diesel migrations run
```


## Example requests

Query for a user with the id value of 1 in postgres:

```
$ curl -H "Content-Type: application/json" localhost:8007/users/1
{"id":1,"username":"bob","email":"bob@no-reply","password":"3d172959deda021453161031486f7e3126f730d80f1f7cb447edbe36777ff0c4113b0508e3cb87c27784ff0e84cb96eb7727a6e6bd597be0bc19436e700eafff"}
```

Insert new data to postgres from JSON:
```
$ curl -X POST -d @add.json -H "Content-Type: application/json" localhost:8007/users
{"id":3,"username":"slanky","email":"slippy@no-reply","password":"be6c20a8a80de1d70a95df3abf17c490e119074db020707e5d1a58255657f372336885580bfb1ae2acfced7d3170d0691669be89c7c266b8c8990e0b766c3ab0"}
````

Update existing data to postgres from JSON:
```
$ curl -X PUT -d @updatepw.json -H "Content-Type: application/json" localhost:8007/users/3
{"id":3,"username":"slanky","email":"slippy@no-reply","password":"85c6e5caccd395656090108e7910f31004f89e30b85ecab8b0fc68cd292541796e5a49803dc43641efff22c1252b190adcd7080e1dda725e7f77acb0ef22a073"}
````

Delete existing data to postgres from JSON:
```
$ curl -X DELETE -d @updatepw.json -H "Content-Type: application/json" localhost:8007/users/3
{"deleted":1}
````



## tips to avoid common mistakes

- ensure that the database credentials and network connectivity are in place first
- ensure that the JSON used matches the schema used in postgres and in the code.

API HTTP body JSON:
```
{"username":"slanky","email":"slippy@no-reply","password":"3d172959deda021453161031486f7e3126f730d80f1f7cb447edbe36777ff0c4113b0508e3cb87c27784ff0e84cb96eb7727a6e6bd597be0bc19436e700eafff"}
```
src/schema.rs:
```

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
```
postgres schema:
```
CREATE TABLE IF NOT EXISTS user (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )

```
