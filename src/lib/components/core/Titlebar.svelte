<script lang="ts">
	import Minus from "lucide-svelte/icons/minus";
	import ChevronUp from "lucide-svelte/icons/chevron-up";
	import ChevronDown from "lucide-svelte/icons/chevron-down";
	import X from "lucide-svelte/icons/x";
	import { getCurrentWindow } from "@tauri-apps/api/window";

	let maximized = $state(false);

	async function minimizeWindow() {
		let window = getCurrentWindow();
		await window.minimize();
	}

	async function maximizeWindow() {
		let window = getCurrentWindow();
		maximized = await window.isMaximized();
		if (maximized) {
			await window.unmaximize();
		} else {
			await window.maximize();
		}
	}

	async function closeWindow() {
		let window = getCurrentWindow();
		await window.close();
	}
</script>

{#snippet titlebarButton(IconComponent: any, onclick: () => void)}
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
