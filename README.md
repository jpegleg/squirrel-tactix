# squirrel-tactix 🐿️ 

An Actix API microservice that uses Diesel Postgresql.

Squirrel tActix is an async actix backend designed for JSON-baesd database management.
It runs as a cloud-native microservice in a tiny OCI image (see Dockerfile) or the rust binary can be run however.

If we are running it in a VM, we could put it behind HAProxy, for example.

## headers logged, pass UUID or transaction id in header for correlation

```
[2022-10-30T06:59:44.250973553Z INFO ] - squirrel-tactix - /users/2 GET function request - from Some(192.168.1.121:38086) - HeaderMap { inner: {"accept": One("*/*"), "content-length": One("0"), "host": One("service-0:8007"), "user-agent": One("curl/7.81.0"), "txid": One("1c9bda74-ff98-4aef-ac7a-d427b2900856"), "content-type": One("application/json")} }

```

## credentials, diesel migration, and initialization

Current version has a `.env` file that contains the database credentials. Example `.env` file that runs on the loopback device:

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

Any headers will be logged, so we can pass a transaction id:

```
$ curl -H "Content-Type: application/json" -H "tid: 1c9bda74-ff98-4aef-ac7a-d427b2900856" localhost:8007/users/1
{"id":1,"username":"bob","email":"bob@no-reply","password":"3d172959deda021453161031486f7e3126f730d80f1f7cb447edbe36777ff0c4113b0508e3cb87c27784ff0e84cb96eb7727a6e6bd597be0bc19436e700eafff"}
```

Any data we want to associate with the request can be set as a header, however such data is optional.

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
```

Example using a random value as the transaction id header and sending a postgres health check request:
```
curl -X GET -H "tid: $(cat /dev/urandom | head -n12 | b2sum | cut -c1-26)" -H "Content-Type: application/json" -d @test.json http://localhost:8007/health
"RECV"
```

And the corresponding STDOUT on the server side, note the event correlation with the header data:
```
[2022-11-02T03:38:19.486992001Z INFO ] - squirrel-tactix - /health GET (health check) request - from Some(127.0.0.1:41426) - HeaderMap { inner: {"accept": One("*/*"), "content-length": One("196"), "host": One("localhost:8007"), "user-agent": One("curl/7.81.0"), "content-type": One("application/json"), "tid": One("0033929ba084f9d1fc9e89b5b6")} }
[2022-11-02T03:38:19.487641671Z INFO ] - squirrel-tactix - /health GET (health check) response OK - HeaderMap { inner: {"accept": One("*/*"), "content-length": One("196"), "host": One("localhost:8007"), "user-agent": One("curl/7.81.0"), "content-type": One("application/json"), "tid": One("0033929ba084f9d1fc9e89b5b6")} }
```

If the postgres query fails, then the server will eventually respond with an internal server error JSON from the error handler.
If the postgres query succeeds, then the health check response is the string "RECV".
Squirrel-tactix does not crash if postgres is taken offline, however postgres connection pool must be able to be built to initialize.
We can take down postgres for maintenance, and squirrel tactix will still hold requests until the timeout (default server timeout is 30 seconds),
which if 30 seconds is passed the error JSON is sent to the client and the CRUD operation is not run. If postgres is down for less than
30 seconds and a request comes through, the request will succeed on the server side as long as that client is still listening: client software
with shorter timeouts than 30 seconds set can get a timeout instead of the error JSON.

## tips to avoid common mistakes

- ensure that the database credentials and network connectivity are in place first
- ensure that the table and schema in postgres (initial migration, etc) have been set up
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
CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )

```
