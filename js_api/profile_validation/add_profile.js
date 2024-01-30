import { Keyring } from '@polkadot/keyring';
import { ApiPromise, WsProvider } from '@polkadot/api';



export async function add_profile(wsprovider, mnemonic, ipfs_string) {
    // console.log(wsprovider);
    return new Promise((resolve, reject) => {
        const wsProvider = new WsProvider(wsprovider);
        const apiPromise = ApiPromise.create({
            provider: wsProvider,
            types: {
                Content: {
                    _enum: {
                        None: 'Null',
                        Other: 'Bytes',
                        IPFS: 'Bytes'
                    }
                }
            }
        });
        const keyring = new Keyring({ type: 'sr25519' });
        const pair = keyring.createFromUri(mnemonic);
        apiPromise.then((api) => {
            const content = api.createType('Content', { 'IPFS': ipfs_string })
            api.tx.profileValidation.addCitizen(content)
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