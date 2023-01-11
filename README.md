HyperDB
=======

Author: **[Afaan Bilal](https://afaan.dev)**

## Introduction
**HyperDB** is an in-memory hyper-fast key-value store written in Rust.

---

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

---

## Configuration
The following environment variables configure the HTTP server.

| Environment Variable | Default value | Description
| :------------------- | :------------ | :-----------
| HYPERDB_HOST         | `0.0.0.0`     | HyperDB HTTP Server Bind Host
| HYPERDB_PORT         | `8765`        | HyperDB HTTP Server Port

---

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
| POST   | /save            | Persist store to file. Returns `OK` on success.
| POST   | /reload          | Reload store from file. Returns `OK` on success.
| DELETE | /reset           | Delete all stored data and clear the persistence on file. Returns `OK` on success.

---

## Clients
| Language    | Source            | Package          | Install
| :---------- | :---------------- | :--------------- | :------
| Javascript  | [HyperDB JS][1]   | [hyperdb-js][2]  | `npm i hyperdb-js`
| PHP         | [HyperDB PHP][3]  | [hyperdb-php][4] | `composer require afaanbilal/hyperdb-php`

[1]: https://github.com/AfaanBilal/hyperdb-js
[2]: https://www.npmjs.com/package/hyperdb-js
[3]: https://github.com/AfaanBilal/hyperdb-php
[4]: https://packagist.org/packages/afaanbilal/hyperdb-php

---

## Benchmarks
Load test using [autocannon](https://github.com/mcollina/autocannon).

### Reading Data (`50,675.2 requests per second`)

`$ autocannon -c 100 -d 30 -p 10 http://127.0.0.1:8765/data/hello`

````
Running 30s test @ http://127.0.0.1:8765/data/hello
100 connections with 10 pipelining factor

┌─────────┬───────┬───────┬───────┬───────┬──────────┬─────────┬───────┐
│ Stat    │ 2.5%  │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev   │ Max   │
├─────────┼───────┼───────┼───────┼───────┼──────────┼─────────┼───────┤
│ Latency │ 15 ms │ 17 ms │ 36 ms │ 39 ms │ 19.39 ms │ 6.59 ms │ 72 ms │
└─────────┴───────┴───────┴───────┴───────┴──────────┴─────────┴───────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg     │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Req/Sec   │ 46559   │ 46559   │ 51039   │ 53791   │ 50675.2 │ 1887.99 │ 46542   │
├───────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ Bytes/Sec │ 5.63 MB │ 5.63 MB │ 6.18 MB │ 6.51 MB │ 6.13 MB │ 229 kB  │ 5.63 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 30

1521k requests in 30.27s, 184 MB read
````

### Writing Data (`49,797.34 requests per second`)

`$ autocannon -c 100 -d 30 -p 10 -b WORLD -m POST http://127.0.0.1:8765/data/hello`

````
Running 30s test @ http://127.0.0.1:8765/data/hello
100 connections with 10 pipelining factor

┌─────────┬───────┬───────┬───────┬───────┬──────────┬─────────┬───────┐
│ Stat    │ 2.5%  │ 50%   │ 97.5% │ 99%   │ Avg      │ Stdev   │ Max   │
├─────────┼───────┼───────┼───────┼───────┼──────────┼─────────┼───────┤
│ Latency │ 14 ms │ 17 ms │ 36 ms │ 38 ms │ 19.74 ms │ 6.95 ms │ 94 ms │
└─────────┴───────┴───────┴───────┴───────┴──────────┴─────────┴───────┘
┌───────────┬─────────┬─────────┬─────────┬─────────┬──────────┬─────────┬─────────┐
│ Stat      │ 1%      │ 2.5%    │ 50%     │ 97.5%   │ Avg      │ Stdev   │ Min     │
├───────────┼─────────┼─────────┼─────────┼─────────┼──────────┼─────────┼─────────┤
│ Req/Sec   │ 43647   │ 43647   │ 49951   │ 52479   │ 49797.34 │ 1497.74 │ 43630   │
├───────────┼─────────┼─────────┼─────────┼─────────┼──────────┼─────────┼─────────┤
│ Bytes/Sec │ 5.28 MB │ 5.28 MB │ 6.05 MB │ 6.35 MB │ 6.03 MB  │ 181 kB  │ 5.28 MB │
└───────────┴─────────┴─────────┴─────────┴─────────┴──────────┴─────────┴─────────┘

Req/Bytes counts sampled once per second.
# of samples: 30

1495k requests in 30.26s, 181 MB read
````

---

## Test
`$ cargo test`

````
    Finished test [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src\main.rs (target\debug\deps\hyperdb-11cc96418dbb140b.exe)

running 12 tests
test hyper::tests::has_file ... ok
test hyper::tests::key_not_present ... ok
test hyper::tests::not_empty ... ok
test hyper::tests::it_clears ... ok
test hyper::tests::key_is_deleted ... ok
test hyper::tests::len_not_zero ... ok
test hyper::tests::key_is_stored ... ok
test hyper::tests::start_from_empty ... ok
test hyper::tests::start_len_zero ... ok
test hyper::tests::value_is_stored ... ok
test hyper::tests::saves_to_file ... ok
test hyper::tests::reloads_from_file ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
````

---

## Contributing
All contributions are welcome. Please create an issue first for any feature request
or bug. Then fork the repository, create a branch and make any changes to fix the bug
or add the feature and create a pull request. That's it!
Thanks!

---

## License
**HyperDB** is released under the MIT License.
Check out the full license [here](LICENSE).
