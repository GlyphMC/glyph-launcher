<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { ProgressBar } from "./ProgressBar.svelte";
	import type { AssetsDownloadState, AssetsDownloadProgress, ProgressEvent } from "$lib/types";
	import { onDestroy, onMount } from "svelte";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";

	let isVisible = $state(false);
	let downloadState = $state<AssetsDownloadState>("none");
	let downloadProgress = $state<AssetsDownloadProgress>({
		assets: 0,
		libraries: 0,
		versionJar: 0
	});

	let statusText = $derived(getStatusText(downloadState));

	let unlistenStarted: UnlistenFn | undefined;
	let unlistenFinished: UnlistenFn | undefined;
	let unlistenProgressAssets: UnlistenFn | undefined;
	let unlistenProgressLibraries: UnlistenFn | undefined;
	let unlistenProgressVersionJar: UnlistenFn | undefined;

	onMount(() => {
		setupEventListeners();
	});

	onDestroy(() => {
		unlistenStarted?.();
		unlistenFinished?.();
		unlistenProgressAssets?.();
		unlistenProgressLibraries?.();
		unlistenProgressVersionJar?.();
	});

	async function setupEventListeners() {
		unlistenStarted = await listen("instance-download-assets-started", () => {
			downloadState = "assets";
			isVisible = true;
		});
		unlistenFinished = await listen("instance-download-assets-finished", () => {
			downloadState = "done";
			isVisible = false;
		});

		unlistenProgressAssets = await setProgressListener("assets");
		unlistenProgressLibraries = await setProgressListener("libraries");
		unlistenProgressVersionJar = await setProgressListener("version-jar");
	}

	async function setProgressListener(name: "assets" | "libraries" | "version-jar") {
		const progressKey: keyof AssetsDownloadProgress = name === "version-jar" ? "versionJar" : name;

		return await listen<ProgressEvent>(`instance-download-${name}-progress`, (event) => {
			downloadProgress[name] = event.payload.percentage;
			downloadState = name;
		});
	}

	function getStatusText(downloadState: AssetsDownloadState): string {
		if (downloadState === "assets") {
			return "Downloading assets";
		} else if (downloadState === "libraries") {
			return "Downloading libraries";
		} else if (downloadState === "version-jar") {
			return "Downloading version jar";
		} else if (downloadState === "done") {
			return "Finished downloading assets";
		} else {
			return "Preparing download...";
		}
	}
</script>

{#if isVisible}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md">
		<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
			<Card.Header>
				<h2 class="text-xl font-bold text-zinc-50">Assets Setup</h2>
				<p class="text-sm text-zinc-50">{statusText}</p>
			</Card.Header>

			<Card.Content>
				<div class="flex flex-col space-y-2">
					{@render ProgressBar(downloadProgress["assets"], "Assets")}
					{@render ProgressBar(downloadProgress["libraries"], "Libraries")}
					{@render ProgressBar(downloadProgress["version-jar"], "Minecraft")}
				</div>
			</Card.Content>
		</Card.Root>
	</div>
{/if}
