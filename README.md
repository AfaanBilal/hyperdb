HyperDB
=======

Author: **[Afaan Bilal](https://afaan.dev)**

## Introduction
**HyperDB** is an in-memory hyper-fast key-value store written in Rust.

## Build and Run
Start the HTTP server: `cargo run`

## Configuration
The following environment variables configure the serve.

| Environment Variable | Default value | Description
| -------------------- | ------------- | -----------
| HYPERDB_HOST         | `127.0.0.1`   | HyperDB HTTP Server Bind Host (Set this to 0.0.0.0 to listen on all)
| HYPERDB_PORT         | `8765`        | HyperDB HTTP Server Port

## Endpoints

| Method | Path             | Description
| ------ | ---------------- | -----------
| GET    | /                | Introduction
| GET    | /ping            | Ping (returns PONG)
| GET    | /has/`{key}`     | Returns `Yes` if `key` is present, otherwise `No`
| GET    | /data/`{key}`    | Returns the value for the `key` if present, otherwise `""`
| POST   | /data/`{key}`    | Sets the value for the `key` to the request body
| DELETE | /data/`{key}`    | Deletes the `key` and any value associated with it
| GET    | /data            | Get all store data
| DELETE | /data            | Delete all stored data
| GET    | /empty           | Returns `Yes` if the store is empty, otherwise `No`

## Contributing
All contributions are welcome. Please create an issue first for any feature request
or bug. Then fork the repository, create a branch and make any changes to fix the bug
or add the feature and create a pull request. That's it!
Thanks!

## License
**HyperDB** is released under the MIT License.
Check out the full license [here](LICENSE).
