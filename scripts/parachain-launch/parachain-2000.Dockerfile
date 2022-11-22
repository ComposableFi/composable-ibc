FROM parachain-node:latest
COPY . /app
RUN /usr/local/bin/parachain-node build-spec --node-key=7c667a99279cf5a884d6bb86fac6e320ef2d413dc880f01ff0e1ad353d615045 > /app/dev.json