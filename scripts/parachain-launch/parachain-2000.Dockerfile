FROM parachain-node:latest
COPY . /app
RUN /usr/local/bin/parachain-node build-spec | sed 's/"para_id": 1000,/"para_id": 2100,/' > /app/tmp.json && \
    /usr/local/bin/parachain-node build-spec --chain /app/tmp.json --raw  --disable-default-bootnode > /app/dev.json && \
    rm /app/tmp.json && \
    /usr/local/bin/parachain-node export-genesis-state --chain /app/dev.json  > /app/genesis_state && \
    /usr/local/bin/parachain-node export-genesis-wasm --chain /app/dev.json  > /app/genesis_wasm
