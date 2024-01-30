import { transfer_balance } from './js_api/balance/balance_tranfer';
import { get_api, get_account_address_from_seed, helloworld } from './js_api/api';
import { add_profile } from './js_api/profile_validation/add_profile';
import { chat_text } from './js_api/ai/chat_hugging_face';
import { add_profile_stake } from './js_api/profile_validation/add_profile_stake';
import { challege_profile } from './js_api/profile_validation/challenge_profile';
import { waitReady } from "@polkadot/wasm-crypto";
import { getAccounts, signPayload } from './js_api/polkadot_extension/extension';
import { getSTSCredentials } from './js_api/4everland/sts_credentials';
import { uploadFile4everland } from './js_api/4everland/upload_file';
import { getCid } from './js_api/ipfs/cid_helia';
import { uploadPinString4everland } from './js_api/4everland/upload_pin';
import { uploadPinBlob4everland } from './js_api/4everland/upload_pin_file';


await waitReady();
export { transfer_balance, get_api, get_account_address_from_seed, helloworld, add_profile, chat_text, add_profile_stake, challege_profile, getAccounts, signPayload, getSTSCredentials, uploadFile4everland, getCid, uploadPinString4everland, uploadPinBlob4everland}













