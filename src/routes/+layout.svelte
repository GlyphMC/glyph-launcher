<script lang="ts">
	import type { Snippet } from "svelte";
	import { ModeWatcher } from "mode-watcher";
	import Titlebar from "$lib/components/core/Titlebar.svelte";
	import "../app.pcss";

	let { children }: { children: Snippet } = $props();
</script>

<ModeWatcher />
<Titlebar />
{@render children()}
