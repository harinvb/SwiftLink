<script>
    import Header from '$lib/Header.svelte';
    import '../app.css'
    import Footer from '$lib/Footer.svelte';
</script>

<div class='app'>
    <Header />

    <main>
        <slot />
    </main>

<Footer/>
</div>
