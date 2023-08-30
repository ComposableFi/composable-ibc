
build-parachain-node:
	docker build -t parachain-node -f scripts/parachain.Dockerfile .

parachain-launch-generate:
	parachain-launch generate  --config ./scripts/parachain-launch/config.yml -o ./scripts/parachain-launch/