# parachain-register

This script registers a parachain.

## Build

```bash
yarn build
```

## Running

```bash
yarn start
```

### Environment variables

| Name                    | Default                 |
| ----------------------- | ----------------------- |
| NODE_URL                | ws://localhost:9944     |
| PARA_ID                 | 2000                    |
| GENESIS_STATE_FILE_PATH | parachain/genesis_state |
| GENESIS_WASM_FILE_PATH  | parachain/genesis_wasm  |

## Docker

### Build a Docker image

```bash
docker build . -t parachain-register
```

### Running using the Docker image

```bash
docker run \
    --rm \
    -ti \
    -u$(id -u):$(id -g) \
    -v$(pwd)/genesis_state:/build/parachain/genesis_state \
    -v$(pwd)/genesis_wasm:/build/parachain/genesis_wasm \
    -eNODE_URL=ws://node-url.example.com:9944 \
    -ePARA_ID=2222 \
    parachain-register
```
