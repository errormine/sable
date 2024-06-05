import { Stronghold } from "@tauri-apps/plugin-stronghold";
import { appDataDir } from "@tauri-apps/api/path";

const initStronghold = async () => {
    const vaultPath = `${await appDataDir()}/vault.hold`;
    const vaultPassword = import.meta.env.VITE_VAULT_KEY;
    const stronghold = await Stronghold.load(vaultPath, vaultPassword);
  
    let client;
    const clientName = 'sable';
    // This throws an error the first time, but it doesn't mean anything!@!!!
    try {
        client = await stronghold.loadClient(clientName);
    } catch (e) {
        client = await stronghold.createClient(clientName);
    }
  
    return {
        stronghold,
        client,
    };
};

const { stronghold, client } = await initStronghold();
const store = client.getStore();

export async function getRecord(key) {
    let data = await store.get(key);
    return new TextDecoder().decode(new Uint8Array(data));
}

export async function insertRecord(key, value) {
    let data = Array.from(new TextEncoder().encode(value));
    await store.insert(key, data);
    await stronghold.save();
}

export async function deleteRecord(key) {
    await store.remove(key);
    await stronghold.save();
}