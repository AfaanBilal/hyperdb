HyperDB
=======

Author: **[Afaan Bilal](https://afaan.dev)**

## Introduction
**HyperDB** is an in-memory hyper-fast key-value store written in Rust.

## Run with Docker
`$ docker run --rm -p 8765:8765 afaanbilal/hyperdb`

````
[HyperDB v0.1.0 (https://afaan.dev)]: Server starting on 0.0.0.0:8765
````

## Build and Run
`$ cargo run`

````
[HyperDB v0.1.0 (https://afaan.dev)]: Server starting on 0.0.0.0:8765
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
| POST   | /save            | Persist store to file.
| POST   | /reload          | Reload store from file.
| DELETE | /reset           | Delete all stored data and clear the persistence on file.

## Benchmark
Load test using [autocannon](https://github.com/mcollina/autocannon).

### Reading Data (`49,785.6 requests per second`)

`$ autocannon -c 100 -d 30 -p 10 http://127.0.0.1:8765/data/hello`

````
100 connections with 10 pipelining factor


┌─────────┬───────┬───────┬───────┬───────┬──────────┬─────────┬───────┐
│ Stat    │ 2.5%  │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev   │ Max   │
├─────────┼───────┼───────┼───────┼───────┼──────────┼─────────┼───────┤
│ Latency │ 15 ms │ 17 ms │ 37 ms │ 39 ms │ 19.78 ms │ 6.75 ms │ 73 ms │
└─────────┴───────┴───────┴───────┴───────┴──────────┴─────────┴───────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 43935   │ 43935   │ 49951   │ 52479   │ 49785.6 │ 1622.59 │ 43904   │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 5.31 MB │ 5.31 MB │ 6.05 MB │ 6.35 MB │ 6.02 MB │ 197 kB  │ 5.31 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 30

1495k requests in 30.31s, 181 MB read
````

### Writing Data (`49,130.67 requests per second`)

`$ autocannon -c 100 -d 30 -p 10 -b WORLD -m POST http://127.0.0.1:8765/data/hello`

````

Running 30s test @ http://127.0.0.1:8765/data/hello
100 connections with 10 pipelining factor


┌─────────┬───────┬───────┬───────┬───────┬──────────┬─────────┬───────┐
│ Stat    │ 2.5%  │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev   │ Max   │
├─────────┼───────┼───────┼───────┼───────┼──────────┼─────────┼───────┤
│ Latency │ 14 ms │ 17 ms │ 37 ms │ 40 ms │ 20.03 ms │ 7.21 ms │ 84 ms │
└─────────┴───────┴───────┴───────┴───────┴──────────┴─────────┴───────┘
┌───────────┬────────┬────────┬─────────┬─────────┬──────────┬─────────┬─────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%     │ 97.5%   │ Avg      │ Stdev   │ Min     │
├───────────┼────────┼────────┼─────────┼─────────┼──────────┼─────────┼─────────┤
│ Req/Sec   │ 42111  │ 42111  │ 49887   │ 51487   │ 49130.67 │ 2026.02 │ 42094   │
├───────────┼────────┼────────┼─────────┼─────────┼──────────┼─────────┼─────────┤
│ Bytes/Sec │ 5.1 MB │ 5.1 MB │ 6.04 MB │ 6.23 MB │ 5.94 MB  │ 245 kB  │ 5.09 MB │
└───────────┴────────┴────────┴─────────┴─────────┴──────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 30

1475k requests in 30.3s, 178 MB read
````

## Test
`$ cargo test`

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
