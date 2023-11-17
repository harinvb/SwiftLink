<script lang="ts">
	import { page } from '$app/stores';
	import { Home, History, UserCog2 } from 'lucide-svelte';

	let currentPage: String | null = '/';
	page.subscribe((value) => {
		currentPage = value.route.id;
	});

	const pages = [
		{ name: 'Config', route: '/conf', icon: UserCog2 },
		{ name: 'Home', route: '/', icon: Home },
		{ name: 'History', route: '/history', icon: History }
	];
</script>

<div class="btm-nav">
	{#each pages as { name, route, icon }}
		<a href={route} class:active={currentPage === route} class="active">
			<svelte:component this={icon} />
			<!-- <span>{name}</span> -->
		</a>
	{/each}
</div>
