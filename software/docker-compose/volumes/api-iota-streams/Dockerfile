FROM ekidd/rust-musl-builder:latest AS builder

COPY  --chown=rust:rust src ./

COPY Cargo.toml /home/rust/
COPY Cargo.lock /home/rust/

RUN cargo build --release

FROM debian:buster
RUN apt-get update && apt-get install -y --no-install-recommends \
		ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /opt/api-iota-streams

COPY --from=builder \
    /home/rust/target/x86_64-unknown-linux-musl/release/api-iota-streams \
    ./

COPY development.env ./

CMD ["/opt/api-iota-streams/api-iota-streams"]

# docker build -t api-iota-streams .
# docker run -p 3030:3030 api-iota-streams