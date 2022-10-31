FROM rust:buster as builder
WORKDIR /app

RUN rustup default nightly-2022-08-05 && \
	rustup target add wasm32-unknown-unknown --toolchain nightly-2022-08-05

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

COPY . .

RUN cargo build --release --locked -p hyperspace

# =============

FROM phusion/baseimage:focal-1.2.0

RUN useradd -m -u 1000 -U -s /bin/sh -d /centauri centauri

COPY --from=builder /app/target/release/hyperspace /usr/local/bin

# checks
RUN ldd /usr/local/bin/hyperspace && \
	/usr/local/bin/hyperspace --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/sbin /usr/share/man

USER centauri
EXPOSE 30333 9933 9944

RUN mkdir /centauri/data

VOLUME ["/centauri/data"]

ENTRYPOINT ["/usr/local/bin/hyperspace"]