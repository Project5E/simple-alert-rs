ARG build_dir=/build

FROM rust:latest AS build
ARG build_dir

WORKDIR $build_dir
RUN rustup component add clippy
RUN rustup component add rustfmt

COPY . $build_dir

RUN cargo build --release

FROM debian:stretch-slim
ARG build_dir
RUN apt-get -qy update && \
    apt-get -qy install ca-certificates libssl1.1 && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build $build_dir/target/release/simple-alert-rs /
USER 1000
ENTRYPOINT ["/bin/bash", "-c", "exec /simple-alert-rs \"$@\"", "--"]