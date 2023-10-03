FROM paritytech/ci-linux:production as build

WORKDIR /code

# Install protoc
ENV PROTOC_ZIP=protoc-3.20.3-linux-x86_64.zip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v3.20.3/${PROTOC_ZIP}
RUN unzip -o ${PROTOC_ZIP} -d ./proto
RUN chmod 755 -R ./proto/bin
ENV BASE=/usr
RUN cp ./proto/bin/protoc ${BASE}/bin/
RUN cp -R ./proto/include/* ${BASE}/include/

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN rustup +nightly target add wasm32-unknown-unknown
RUN cargo build --release -p parachain-node --features=testing

FROM phusion/baseimage:focal-1.2.0
WORKDIR /node

# Copy the node binary.
COPY --from=build /code/target/release/parachain-node /usr/local/bin

# Install root certs, see: https://github.com/paritytech/substrate/issues/9984
RUN apt update && \
    apt install -y ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -U -s /bin/sh -d /parachain parachain && \
    chown -R parachain:parachain /usr/local/bin

USER parachain

# check if executable works in this container
RUN /usr/local/bin/parachain-node --version

EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/parachain-node"]
