<script>
	export const prerender = true;
	export const ssr = false;
	import Header from '$lib/Header.svelte';
	import '../app.css';
	//    import Footer from '$lib/Footer.svelte';
</script>

<Header />
<main class="h-fit flex items-center justify-center">
	<slot />
</main>
<!--<Footer />-->
