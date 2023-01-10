HyperDB
=======

Author: **[Afaan Bilal](https://afaan.dev)**

## Introduction
**HyperDB** is an in-memory hyper-fast key-value store written in Rust.

## Build and Run
Start the HTTP server: `cargo run`
````
[HyperDB v0.1.0 (https://afaan.dev)]: Server starting on 127.0.0.1:8765
````

## Configuration
The following environment variables configure the serve.

| Environment Variable | Default value | Description
| :------------------- | :------------ | :-----------
| HYPERDB_HOST         | `0.0.0.0`     | HyperDB HTTP Server Bind Host
| HYPERDB_PORT         | `8765`        | HyperDB HTTP Server Port

## Endpoints

| Method | Path             | Description
| :----- | :--------------- | :-----------
| GET    | /                | Version. Example: `[HyperDB v0.1.0 (https://afaan.dev)]`.
| GET    | /ping            | Ping (returns `PONG`).
| GET    | /has/`{key}`     | Returns `Yes` if `key` is present, otherwise `No`.
| GET    | /data/`{key}`    | Returns the value for the `key` if present, otherwise `""`.
| POST   | /data/`{key}`    | Sets the value for the `key` to the request body.
| DELETE | /data/`{key}`    | Deletes the `key` and any value associated with it. Returns `OK` on success.
| GET    | /data            | Get all stored data.
| DELETE | /data            | Delete all stored data. Returns `OK` on success.
| GET    | /empty           | Returns `Yes` if the store is empty, otherwise `No`.

## Test
Test HyperDB: `cargo test`
````
    Finished test [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src\main.rs (target\debug\deps\hyperdb-6884b447c6b75f1d.exe)

running 10 tests
test hyper::tests::has_file ... ok
test hyper::tests::it_clears ... ok
test hyper::tests::start_from_empty ... ok
test hyper::tests::key_is_deleted ... ok
test hyper::tests::key_not_present ... ok
test hyper::tests::key_is_stored ... ok
test hyper::tests::not_empty ... ok
test hyper::tests::len_not_zero ... ok
test hyper::tests::start_len_zero ... ok
test hyper::tests::value_is_stored ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
````

## Contributing
All contributions are welcome. Please create an issue first for any feature request
or bug. Then fork the repository, create a branch and make any changes to fix the bug
or add the feature and create a pull request. That's it!
Thanks!

## License
**HyperDB** is released under the MIT License.
Check out the full license [here](LICENSE).
