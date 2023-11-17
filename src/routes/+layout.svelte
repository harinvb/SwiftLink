<script>
	import NavBar from '$lib/NavBar.svelte';
	import '../app.css';
	//    import Footer from '$lib/Footer.svelte';
</script>

<!-- <main class="h-full flex items-center justify-center"></main> -->

<div class="flex flex-col h-screen">
	<!-- Top div occupying the entire screen above the navbar -->
	<div class="flex-grow">
		<!-- Content of the top div goes here -->
		<slot />
	</div>

	<NavBar />
</div>
