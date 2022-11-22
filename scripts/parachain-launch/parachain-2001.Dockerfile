FROM parachain-node:latest
COPY . /app
RUN /usr/local/bin/parachain-node build-spec --node-key=e5ad8b579451f25035feeccaee89468a7fba53fbce35d27afe5fdbbf412722e9 > /app/dev.json