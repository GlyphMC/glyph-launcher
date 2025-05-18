<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { ProgressBar } from "$lib/components/core/ProgressBar.svelte";
	import { type UnlistenFn } from "@tauri-apps/api/event";
	import { scale } from "svelte/transition";
	import { quintOut } from "svelte/easing";
	import { onDestroy, onMount } from "svelte";
	import { events } from "$lib/bindings";

	type AssetsDownloadState = "none" | "assets" | "libraries" | "version-jar" | "done";
	interface AssetsDownloadProgress {
		assets: number;
		libraries: number;
		versionJar: number;
	}

	let isVisible = $state(false);
	let downloadState = $state<AssetsDownloadState>("none");
	let progress = $state<AssetsDownloadProgress>({
		assets: 0,
		libraries: 0,
		versionJar: 0
	});

	let statusText = $derived(getStatusText(downloadState));
	let unlistenFns: UnlistenFn[] = [];

	onMount(() => setupEventListeners());
	onDestroy(() => {
		unlistenFns.forEach((unlisten) => unlisten());
		unlistenFns = [];
	});

	async function setupEventListeners() {
		unlistenFns.forEach((unlisten) => unlisten());
		unlistenFns = [];

		unlistenFns.push(
			await events.assetsDownloadStartedEvent.listen((event) => {
				downloadState = "assets";
				progress = { assets: 0, libraries: 0, versionJar: 0 };
				isVisible = true;
			})
		);

		unlistenFns.push(
			await events.assetsDownloadFinishedEvent.listen(() => {
				downloadState = "done";
				progress.assets = 100;
				progress.libraries = 100;
				progress.versionJar = 100;

				setTimeout(() => (isVisible = false), 1500);
			})
		);

		unlistenFns.push(
			await events.assetProgressEvent.listen((event) => {
				const { kind, percentage } = event.payload;

				if (kind === "Assets") {
					progress.assets = percentage;
					if (percentage < 100) downloadState = "assets";
				} else if (kind === "Libraries") {
					progress.libraries = percentage;
					if (percentage < 100) downloadState = "libraries";
				} else if (kind === "version-jar") {
					progress.versionJar = percentage;
					if (percentage < 100) downloadState = "version-jar";
				}
			})
		);
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
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md"
		in:scale={{ duration: 300, start: 0.5, opacity: 0, easing: quintOut }}
		out:scale={{ duration: 200, opacity: 0, easing: quintOut }}>
		<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
			<Card.Header>
				<h2 class="text-xl font-bold text-zinc-50">Assets Setup</h2>
				<p class="text-sm text-zinc-50">{statusText}</p>
			</Card.Header>

			<Card.Content>
				<div class="flex flex-col space-y-2">
					{@render ProgressBar(progress.assets, "Assets")}
					{@render ProgressBar(progress.libraries, "Libraries")}
					{@render ProgressBar(progress.versionJar, "Minecraft")}
				</div>
			</Card.Content>
		</Card.Root>
	</div>
{/if}
