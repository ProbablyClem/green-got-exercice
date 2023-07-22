FROM rust:buster as build

RUN apt-get update && apt-get -y install cmake protobuf-compiler
# create a new empty shell project
RUN USER=root cargo new --bin service
WORKDIR /service

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/service*
RUN cargo build --release

# our final base
FROM rust:1.49

# copy the build artifact from the build stage
COPY --from=build /service/target/release/service .

EXPOSE 3000
# set the startup command to run your binary
CMD ["./service"]
