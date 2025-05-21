<script lang="ts">
	import { page } from "$app/state";
	import { commands, events, type Screenshot } from "$lib/bindings";
	import { onDestroy, onMount } from "svelte";
	import type { UnlistenFn } from "@tauri-apps/api/event";
	import { Button } from "$lib/components/ui/button";
	import { open } from "@tauri-apps/plugin-shell";

	async function fetchScreenshots(instanceSlug: string): Promise<Screenshot[]> {
		const res = await commands.getScreenshots(instanceSlug);
		if (res.status === "ok") {
			return res.data;
		}
		throw new Error(res.error);
	}

	async function startScreenshotWatcher(instanceSlug: string, onNewScreenshot: () => void): Promise<UnlistenFn> {
		await commands.watchScreenshotsForInstance(instanceSlug);
		return await events.screenshotEvent.listen(onNewScreenshot);
	}

	let screenshots: Screenshot[] = $state([]);
	let screenshotsPromise: Promise<Screenshot[]> = $state(Promise.resolve([]));
	let unlisten: UnlistenFn | undefined;
	const instanceSlug = page.params.slug;

	onMount(() => {
		screenshotsPromise = fetchScreenshots(instanceSlug).then((data) => {
			screenshots = data;
			return data;
		});

		startScreenshotWatcher(instanceSlug, async () => {
			const newScreenshots = await fetchScreenshots(instanceSlug);
			screenshots = newScreenshots;
		}).then((unlistenFn) => {
			unlisten = unlistenFn;
		});
	});

	onDestroy(() => {
		unlisten?.();
		commands.stopWatchingScreenshots();
	});

	async function openScreenshotsDir() {
		await commands.openScreenshotsDir(instanceSlug);
	}
</script>

{#snippet screenshotCard(screenshot: Screenshot)}
	<div class="flex flex-col overflow-hidden rounded-lg border border-zinc-300 bg-white shadow-lg dark:border-zinc-700 dark:bg-zinc-800">
		<div class="overflow-hidden bg-zinc-200 dark:bg-zinc-700">
			<img src={screenshot.data} alt="Screenshot: {screenshot.name}" class="block w-full object-contain" loading="lazy" />
		</div>
		<div class="px-2 py-1">
			<p class="truncate text-xs text-zinc-600 dark:text-zinc-400" title={screenshot.name}>
				{screenshot.name}
			</p>
		</div>
	</div>
{/snippet}

<div class="h-full w-full overflow-y-auto bg-background p-4 text-foreground lg:px-6 lg:py-5">
	{#await screenshotsPromise}
		<div class="flex h-full items-center justify-center">
			<p class="text-lg text-zinc-400">Loading screenshots...</p>
		</div>
	{:then}
		{#if screenshots.length > 0}
			<div class="grid grid-cols-1 gap-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
				{#each screenshots as screenshot}
					{@render screenshotCard(screenshot)}
				{/each}
			</div>

			<Button class="fixed bottom-6 right-6" onclick={openScreenshotsDir}>Open Folder</Button>
		{:else}
			<p class="text-center text-lg">No screenshots available for this instance.</p>
		{/if}
	{:catch error}
		<div class="flex h-full items-center justify-center">
			<p class="text-lg text-red-500">Failed to load screenshots: {error.message}</p>
		</div>
	{/await}
</div>
