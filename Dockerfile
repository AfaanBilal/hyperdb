#
# HyperDB
#
# In-memory hyper-fast key-value store.
#
# @author Afaan Bilal
# @link   https://afaan.dev
#

#
# Stage 1 (Build)
#

FROM rust:1.66-slim-buster AS build

WORKDIR /hyperdb

COPY . .

RUN cargo build --release

#
# Stage 2 (Run)
#

FROM debian:buster-slim

WORKDIR /hyperdb

COPY --from=build /hyperdb/target/release/hyperdb ./hyperdb

EXPOSE 8765

# And away we go...
CMD [ "./hyperdb" ]
