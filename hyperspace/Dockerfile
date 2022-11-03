FROM rustlang/rust:nightly-buster as backend-builder

WORKDIR /usr/src/hyperspace-build
COPY . .

ENV PATH="${PATH}:/usr/src/hyperspace-build/.local/bin"

# Protobuff issue with version 3 and optional features
# Installing a specific version that works locally for Blas
RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v3.12.4/protoc-3.12.4-linux-x86_64.zip && \
    unzip protoc-3.12.4-linux-x86_64.zip -d .local && \
    protoc --version && \
    make build-release-hyperspace && \
    ls -l ./target/release

FROM debian:buster-slim

COPY --from=backend-builder /usr/src/hyperspace-build/target/release/hyperspace .
ENTRYPOINT ["./hyperspace"]
 