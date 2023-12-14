FROM paritytech/ci-unified:bullseye-1.73.0-2023-05-23 as builder

WORKDIR /code

COPY . .

ENV PATH="/usr/local/protoc/bin:$PATH"

RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-x86_64.zip && \
	unzip  protoc-21.9-linux-x86_64.zip -d /usr/local/protoc && \
	protoc --version


# The .wasm file will live in /code/target/wasm32-unknown-unknown/release/ics10_grandpa_cw.wasm
RUN RUSTFLAGS='-C link-arg=-s' cargo build -p ics10-grandpa-cw --target=wasm32-unknown-unknown --release --lib