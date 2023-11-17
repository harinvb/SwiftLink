import { invoke } from '@tauri-apps/api/tauri';
import type { PageData, PageLoad } from './$types';

export async function load(params: PageLoad) {
	let server_data = (await invoke('get_discovered_clients')) as Array<String>;
	return { clients: server_data };
}
