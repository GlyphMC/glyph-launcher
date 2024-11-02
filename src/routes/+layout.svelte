<script lang="ts">
	import Sidebar from "$lib/Sidebar.svelte";
	import Titlebar from "$lib/Titlebar.svelte";
	import "../app.css";

	let { children } = $props();
</script>

<Sidebar />

<section class="ml-80 pb-12">
	<Titlebar />
	{@render children()}
</section>
