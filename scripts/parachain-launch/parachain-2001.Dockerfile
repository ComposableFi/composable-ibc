FROM parachain-node:latest
COPY . /app
RUN /usr/local/bin/parachain-node build-spec | sed 's/"para_id": 1000,/"para_id": 2001,/' > /app/tmp.json && \
    /usr/local/bin/parachain-node build-spec --chain /app/tmp.json --raw  --disable-default-bootnode > /app/dev.json && \
    rm /app/tmp.json
