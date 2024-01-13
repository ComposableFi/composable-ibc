FROM paritytech/ci-unified:bullseye-1.73.0-2023-05-23 as builder

WORKDIR /code

COPY . .

ENV PATH="/usr/local/protoc/bin:$PATH"

RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-x86_64.zip && \
	unzip  protoc-21.9-linux-x86_64.zip -d /usr/local/protoc && \
	protoc --version

RUN git config --global url."https://github.com/foundry-rs/forge-std.git".insteadOf "git@github.com:foundry-rs/forge-std"

RUN cargo build --release --bin evm-indexer

# =============

FROM debian:bullseye-slim

RUN useradd -m -u 1000 -U -s /bin/sh -d /centauri centauri

COPY --from=builder /code/hyperspace/ethereum/evm-indexer/.env /usr/local/bin
COPY --from=builder /code/target/release/evm-indexer /usr/local/bin

# add ca certificates so that it works with ssl endpoints
RUN apt update && \
	apt install -y ca-certificates

# checks
RUN ldd /usr/local/bin/evm-indexer && \
	/usr/local/bin/evm-indexer --help

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/sbin /usr/share/man

USER centauri

RUN mkdir /centauri/data

VOLUME ["/centauri/data"]

ENTRYPOINT ["/usr/local/bin/evm-indexer"]
