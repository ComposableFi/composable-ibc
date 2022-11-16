FROM parachain-node:latest
COPY . /app
RUN /usr/local/bin/parachain-node build-spec > /app/dev.json