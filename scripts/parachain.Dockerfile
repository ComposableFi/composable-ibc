FROM paritytech/ci-linux:production as build

WORKDIR /code

RUN apt install protobuf-compiler

COPY . .

RUN cargo build --release -p parachain-node

FROM ubuntu:20.04
WORKDIR /node

# Copy the node binary.
COPY --from=build /code/target/release/parachain-node .

# Install root certs, see: https://github.com/paritytech/substrate/issues/9984
RUN apt update && \
    apt install -y ca-certificates && \
    update-ca-certificates && \
    apt remove ca-certificates -y && \
    rm -rf /var/lib/apt/lists/*

EXPOSE 30333 9933 9944

ENTRYPOINT ["parachain-node"]