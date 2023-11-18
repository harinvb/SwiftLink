<script lang="ts">
	import '../app.css';
	import { listen } from '@tauri-apps/api/event';
	import type { PageData } from './$types';

	export let data: PageData;
	listen('test', (event: unknown) => {
		data.clients = [...data.clients, event.payload];
	});
</script>


<section class="h-fit flex items-center justify-center">
	<p>Home Page</p>

	{#each data.clients as item}
		<p>{item}</p>
	{/each}
</section>
