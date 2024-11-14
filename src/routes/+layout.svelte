<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index";
	import AppSidebar from "$lib/ui/Sidebar.svelte";
	import Titlebar from "$lib/ui/Titlebar.svelte";
	import { ModeWatcher } from "mode-watcher";
	import "../app.pcss";

	let { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider open={true} class="bg-background">
	<AppSidebar />
	<Sidebar.Trigger class="z-50 flex h-12 items-center justify-between px-4 hover:bg-transparent" />
	{@render children()}
	<section class="pb-12">
		<Titlebar />
	</section>
</Sidebar.Provider>
