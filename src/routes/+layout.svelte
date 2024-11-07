<script lang="ts">
	import Sidebar from "$lib/ui/Sidebar.svelte";
	import Titlebar from "$lib/ui/Titlebar.svelte";
	import "../app.css";

	let { children } = $props();
</script>

<Sidebar />

<section class="ml-80 pb-12">
	<Titlebar />
	{@render children()}
</section>
