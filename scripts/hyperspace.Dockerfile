FROM paritytech/ci-linux:production as build

WORKDIR /code

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

RUN mkdir /centauri/data

VOLUME ["/centauri/data"]

ENTRYPOINT ["/usr/local/bin/hyperspace"]