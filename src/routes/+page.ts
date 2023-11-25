
import { invoke } from '@tauri-apps/api/primitives';

export async function load() {
	const server_data = (await invoke('get_discovered_clients')) as Array<string>;
	return { clients: server_data };
}
