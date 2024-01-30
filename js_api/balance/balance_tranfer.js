import { Keyring } from '@polkadot/keyring';
import { waitReady } from "@polkadot/wasm-crypto";
import { get_api } from './../api';

export async function transfer_balance(wsprovider, mnemonic, credit_user, value) {
    await waitReady();
    return new Promise((resolve, reject) => {
        const apiPromise = get_api(wsprovider);
        const keyring = new Keyring({ type: 'sr25519' });
        const pair = keyring.createFromUri(mnemonic);
        apiPromise.then((api) => {
            api.tx.balances.transfer_allow_death(credit_user, value)
                .signAndSend(pair, ({ status, dispatchError }) => {
                    if (dispatchError) {
                        if (dispatchError.isModule) {
                            const decoded = api.registry.findMetaError(dispatchError.asModule);
                            const { docs, name, section } = decoded;
                            reject(`Error ${section}.${name}: ${docs.join(' ')}`);
                        } else {
                            reject(`Error ${dispatchError.toString()}`);
                        }
                    } else if (status.isFinalized) {
                        resolve(`Finalized ${status.asFinalized.toString()}`);
                    }
                })
                .catch(reject);
        }).catch(reject);
    });
}



