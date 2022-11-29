import { readFile } from 'fs/promises';
import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/keyring";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { ISubmittableResult, RegistryTypes } from "@polkadot/types/types";


async function main() {
    const url = process.env.NODE_URL ?? "ws://localhost:9944";
    const paraId = Number(process.env.PARA_ID ?? 2000);
    const genesisStateFile = process.env.GENESIS_STATE_FILE_PATH ?? "parachain/genesis_state";
    const genesisWasmFile = process.env.GENESIS_WASM_FILE_PATH ?? "parachain/genesis_wasm";
    console.log(
        `node url: ${url}, para id: ${paraId}, `
        + `genesis state: ${genesisStateFile}, genesis wasm: ${genesisWasmFile}`
    );

    const genesisState = (await readFile(genesisStateFile)).toString();
    const genesisWasm = (await readFile(genesisWasmFile)).toString();

    const api = await createApi(url, {});

    await chainInfo(api);

    const keyring = new Keyring({ type: "sr25519" });
    const root = keyring.addFromUri("//Alice", { name: "Alice default" });
    await register(api, root, paraId, genesisState, genesisWasm);
}

async function createApi(url: string, types: RegistryTypes | undefined): Promise<ApiPromise> {
    const provider = new WsProvider(url);
    return await ApiPromise.create({ provider, types });
}

async function chainInfo(api: ApiPromise) {
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version(),
    ]);

    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
}

async function register(
    api: ApiPromise,
    sudoAcc: KeyringPair,
    paraId: number,
    genesisState: string,
    genesisWasm: string,
) {
    return new Promise(async (resolvePromise, _reject) => {
        const paraGenesisArgs = {
            genesis_head: genesisState,
            validation_code: genesisWasm,
            parachain: true,
        };
        await api.tx.sudo
            .sudo(api.tx.parasSudoWrapper.sudoScheduleParaInitialize(paraId, paraGenesisArgs))
            .signAndSend(sudoAcc, ({ status }: ISubmittableResult) => {
                console.log(`Current status is ${status}`);
                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    resolvePromise(0)
                }
            });
    });
}

main().catch(console.error).finally(() => process.exit());
