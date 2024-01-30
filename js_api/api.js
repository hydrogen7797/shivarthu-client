import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

export async function get_api(wsprovider) {
    const wsProvider = new WsProvider(wsprovider);
    const api = await ApiPromise.create({ provider: wsProvider });
    return api;

}

export function get_account_address_from_seed(mnemonic) {
    // console.log(mnemonic);  
    const keyring = new Keyring({ type: 'sr25519' });
    const pair = keyring.createFromUri(mnemonic);
    // console.log("pair address", pair.address);
    return pair.address;
}

export async function helloworld() {
    const wsProvider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    console.log(api.genesisHash.toHex());
}
