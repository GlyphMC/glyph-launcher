<script lang="ts">
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { page } from "$app/state";
	import type { Attachment } from "svelte/attachments";
	import { events } from "$lib/bindings";

	let logMessages = $state<string[]>([]);
	let wrapLines = $state(true);

	const slug = page.params.slug;

	function toggleLineWrap() {
		wrapLines = !wrapLines;
	}

	function clearLogs() {
		logMessages = [];
	}

	const scrollToBottom: Attachment = (element) => {
		const eventName = `${slug}-log`.replaceAll(".", "_");
		let unlistenFn: UnlistenFn | null = null;

		const setupListener = async () => {
			unlistenFn = await events.instanceLogEvent.listen((event) => {
				const { line } = event.payload;
				logMessages.push(line);

				setTimeout(() => {
					element.scrollTop = element.scrollHeight;
				}, 0);
			});
		};

		setupListener();

		return () => unlistenFn?.();
	};
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
		class="flex-grow select-none overflow-y-auto rounded bg-[#181825] p-[8px]"
		class:whitespace-pre-wrap={wrapLines}
		class:break-all={wrapLines}
		class:whitespace-pre={!wrapLines}
		class:overflow-x-auto={!wrapLines}
		{@attach scrollToBottom}>
		{#if logMessages.length === 0}
			<p class="pt-[20px] text-center text-[#6c7086]">Waiting for logs...</p>
		{/if}
		{#each logMessages as message, i (i)}
			<div class="mb-[4px] leading-snug">{message}</div>
		{/each}
	</div>
</div>
