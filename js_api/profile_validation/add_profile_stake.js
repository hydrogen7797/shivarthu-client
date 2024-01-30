import { Keyring } from '@polkadot/keyring';
import { ApiPromise, WsProvider } from '@polkadot/api';

export async function add_profile_stake(wsprovider, mnemonic, profile_user_account, amount_to_fund) {
    return new Promise((resolve, reject) => {
        const wsProvider = new WsProvider(wsprovider);
        const apiPromise = ApiPromise.create({
            provider: wsProvider
        });
        const keyring = new Keyring({ type: 'sr25519' });
        const pair = keyring.createFromUri(mnemonic);
        apiPromise.then((api) => {
            api.tx.profileValidation.addProfileStake(profile_user_account, amount_to_fund )
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