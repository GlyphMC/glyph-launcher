<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar";
	import AppSidebar from "$lib/components/core/Sidebar.svelte";
	import { page } from "$app/state";
	import { scale } from "svelte/transition";
	import { quintOut } from "svelte/easing";

	let { children } = $props();
	let launcherSectionKey = $derived(getLauncherSectionKey(page.url.hash));

	function getLauncherSectionKey(hash: string): string {
		const instanceMatch = hash.match(/^#\/launcher\/instance\/([^/]+)/);
		if (instanceMatch) {
			return instanceMatch[0];
		}

		const generalLauncherMatch = hash.match(/^#\/launcher\/([^/]+)/);
		if (generalLauncherMatch) {
			return generalLauncherMatch[0];
		}

		return hash;
	}
</script>

<Sidebar.Provider open={true} class="bg-background">
	<AppSidebar />
	<Sidebar.Trigger class="z-50 flex h-12 items-center justify-between px-4 hover:bg-transparent" />
	{#key launcherSectionKey}
		<div class="h-full w-full" in:scale={{ start: 0.95, duration: 250, easing: quintOut }}>
			{@render children()}
		</div>
	{/key}
</Sidebar.Provider>
