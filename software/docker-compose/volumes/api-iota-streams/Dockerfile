FROM rust:latest

WORKDIR /opt/api-iota-streams

COPY --chown=root:root src /opt/api-iota-streams/src
COPY Cargo.toml /opt/api-iota-streams

RUN cargo build --release

WORKDIR /opt/api-iota-streams/target/release/

RUN chmod +X api-iota-streams

COPY development.env /opt/api-iota-streams/target/release/

ENTRYPOINT ["/opt/api-iota-streams/target/release/api-iota-streams"]