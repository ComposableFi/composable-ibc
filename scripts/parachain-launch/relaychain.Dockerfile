FROM seunlanlege/centauri-polkadot:v0.9.27
COPY . /app
RUN /usr/local/bin/polkadot build-spec > /app/rococo-local.json