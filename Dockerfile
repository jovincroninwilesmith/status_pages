# select image
FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin dave-status-pages
WORKDIR /dave-status-pages

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

RUN cargo clean

# build for release
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /dave-status-pages/target/release/status-pages .

# set the startup command to run your binary
CMD ["./status-pages"]
