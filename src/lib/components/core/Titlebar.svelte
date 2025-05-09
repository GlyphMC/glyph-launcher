<script lang="ts">
	import Minus from "lucide-svelte/icons/minus";
	import ChevronUp from "lucide-svelte/icons/chevron-up";
	import ChevronDown from "lucide-svelte/icons/chevron-down";
	import X from "lucide-svelte/icons/x";
	import { Component } from "lucide-svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";

	let maximized = $state(false);

	onMount(() => {
		// Check if the window is maximized on mount
		getCurrentWindow()
			.isMaximized()
			.then((isMaximized) => {
				maximized = isMaximized;
			});
	});

	async function minimizeWindow() {
		const window = getCurrentWindow();
		await window.minimize();
	}

	async function maximizeWindow() {
		const window = getCurrentWindow();
		const isCurrentlyMaximized = await window.isMaximized();
		if (isCurrentlyMaximized) {
			await window.unmaximize();
		} else {
			await window.maximize();
		}
		maximized = !isCurrentlyMaximized;
	}

	async function closeWindow() {
		let window = getCurrentWindow();
		await window.close();
	}
</script>

{#snippet titlebarButton(IconComponent: typeof Component, onclick: () => void)}
	<button class="text-zinc-100 duration-150 hover:text-zinc-50" {onclick}>
		<IconComponent class="h-5 w-5" />
	</button>
{/snippet}

<div class="fixed right-4 top-0 z-40 flex w-full justify-end gap-4 py-2" data-tauri-drag-region>
	{@render titlebarButton(Minus, async () => await minimizeWindow())}
	{#if maximized}
		{@render titlebarButton(ChevronDown, async () => await maximizeWindow())}
	{:else}
		{@render titlebarButton(ChevronUp, async () => await maximizeWindow())}
	{/if}
	{@render titlebarButton(X, async () => await closeWindow())}
</div>
