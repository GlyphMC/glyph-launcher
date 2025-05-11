<script lang="ts">
	import { onMount, onDestroy } from "svelte";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { page } from "$app/state";
	import type { Payload } from "$lib/types";

	let logMessages = $state<string[]>([]);
	let logContainerElement: HTMLDivElement | null = $state(null);
	let unlistenFn: UnlistenFn | null = null;
	let wrapLines = $state(true);

	const slug = page.params.slug;
	const eventName = `${slug}-log`.replaceAll(".", "_");

	function toggleLineWrap() {
		wrapLines = !wrapLines;
	}

	function clearLogs() {
		logMessages = [];
	}

	onMount(() => {
		console.log(`Log page mounted for slug: ${slug}. Listening for event: ${eventName}`);

		(async () => {
			try {
				unlistenFn = await listen<Payload>(eventName, (event) => {
					if (event.payload.message) {
						logMessages = [...logMessages, event.payload.message];
						if (logContainerElement) {
							setTimeout(() => {
								logContainerElement!.scrollTop = logContainerElement!.scrollHeight;
							}, 0);
						}
					} else {
						console.warn("Received log event with unexpected payload:", event.payload);
					}
				});
				console.log(`Successfully listening to ${eventName}`);
			} catch (e) {
				console.error(`Failed to listen to event ${eventName}:`, e);
			}
		})();

		return () => {
			if (unlistenFn) {
				console.log(`Unlistening from ${eventName}`);
				unlistenFn();
			}
		};
	});

	onDestroy(() => {
		if (unlistenFn) {
			console.log(`Log page destroyed. Unlistening from ${eventName}`);
			unlistenFn();
		}
	});
</script>

<div class="flex h-screen flex-col overflow-hidden bg-[#1e1e2e] p-[10px] font-mono text-[#cdd6f4]">
	<div class="mb-[10px] flex items-center border-b border-[#313244] pb-[10px]">
		<h1 class="m-0 text-xl text-[#89b4fa]">Logs for: {slug}</h1>
		<button
			onclick={toggleLineWrap}
			class="z-50 ml-4 h-7 rounded bg-[#45475a] px-3 py-1.5 text-xs text-[#cdd6f4] hover:bg-[#585b70] focus:outline-none focus:ring-2 focus:ring-[#89b4fa]">
			{wrapLines ? "Disable" : "Enable"} Line Wrap
		</button>
		<button
			onclick={clearLogs}
			class="z-50 ml-2 h-7 rounded bg-[#f38ba8] px-3 py-1.5 text-xs text-[#1e1e2e] hover:bg-[#eba0ac] focus:outline-none focus:ring-2 focus:ring-[#f38ba8]">
			Clear Logs
		</button>
	</div>
	<div
		bind:this={logContainerElement}
		class="flex-grow overflow-y-auto rounded bg-[#181825] p-[8px]"
		class:whitespace-pre-wrap={wrapLines}
		class:break-all={wrapLines}
		class:whitespace-pre={!wrapLines}
		class:overflow-x-auto={!wrapLines}>
		{#if logMessages.length === 0}
			<p class="pt-[20px] text-center text-[#6c7086]">Waiting for logs...</p>
		{/if}
		{#each logMessages as message, i (i)}
			<div class="mb-[4px] leading-snug">{message}</div>
		{/each}
	</div>
</div>
