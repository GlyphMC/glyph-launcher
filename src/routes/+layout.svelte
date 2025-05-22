<script lang="ts">
	import { onDestroy, onMount, type Snippet } from "svelte";
	import { ModeWatcher } from "mode-watcher";
	import Titlebar from "$lib/components/core/Titlebar.svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	import "../app.css";

	let { children }: { children: Snippet } = $props();

	// Disable right-click context menu
	onMount(() => document.addEventListener("contextmenu", (event) => event.preventDefault()));
	onDestroy(() => document.removeEventListener("contextmenu", (event) => event.preventDefault()));
</script>

<ModeWatcher />
<Titlebar />
<Toaster closeButton richColors expand />
{@render children()}
