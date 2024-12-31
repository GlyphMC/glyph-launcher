<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import AppSidebar from "$lib/components/core/Sidebar.svelte";

	let { children } = $props();
</script>

<Sidebar.Provider open={true} class="bg-background">
	<AppSidebar />
	<Sidebar.Trigger class="z-50 flex h-12 items-center justify-between px-4 hover:bg-transparent" />
	{@render children()}
</Sidebar.Provider>
