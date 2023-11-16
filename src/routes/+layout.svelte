<script>
    import Header from '$lib/Header.svelte';
</script>

<div class='app'>
    <Header />

    <main>
        <slot />
    </main>

    <footer>
        <p>footer</p>
    </footer>
</div>

<style>
</style>
