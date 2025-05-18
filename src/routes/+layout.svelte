<script lang="ts">
	import { onDestroy, onMount, type Snippet } from "svelte";
	import { ModeWatcher } from "mode-watcher";
	import Titlebar from "$lib/components/core/Titlebar.svelte";
	import "../app.css";

	let { children }: { children: Snippet } = $props();

	// Disable right-click context menu
	onMount(() => document.addEventListener("contextmenu", (event) => event.preventDefault()));
	onDestroy(() => document.removeEventListener("contextmenu", (event) => event.preventDefault()));
</script>

<ModeWatcher />
<Titlebar />
{@render children()}
