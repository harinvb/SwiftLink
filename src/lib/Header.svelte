<script>
	//	import { page } from '$app/stores';
	import '../app.css';
	import SearchButton from '$lib/SearchButton.svelte';
	import NotificationsButton from '$lib/NotificationsButton.svelte';
</script>

<header>
	<div class="navbar bg-base-100">
		<div class="navbar-start">
			<div class="dropdown">
				<!-- svelte-ignore a11y-label-has-associated-control -->
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<label tabindex="0" class="btn btn-ghost btn-circle">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 6h16M4 12h16M4 18h7"
						/>
					</svg>
				</label>
				<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
				<ul
					tabindex="0"
					class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
				>
					<li><a href="/profile">Profile</a></li>
					<li><a href="/history">History</a></li>
					<li><a href="/about">About</a></li>
				</ul>
			</div>
		</div>
		<div class="navbar-center">
			<a class="btn btn-ghost text-xl" href="/">Home</a>
		</div>
		<div class="navbar-end">
			<SearchButton />
			<!-- <NotificationsButton /> -->
		</div>
	</div>
</header>

<style>
</style>
