<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { ProgressBar } from "$lib/components/core/ProgressBar.svelte";
	import type { AssetsDownloadState, AssetsDownloadProgress, ProgressEvent } from "$lib/types";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import type { Attachment } from "svelte/attachments";

	let isVisible = $state(false);
	let downloadState = $state<AssetsDownloadState>("none");
	let progress = $state<AssetsDownloadProgress>({
		assets: 0,
		libraries: 0,
		versionJar: 0
	});

	let statusText = $derived(getStatusText(downloadState));

	const downloadProgress: Attachment = () => {
		let unlistenFns: UnlistenFn[] = [];

		const setupEventListeners = async () => {
			unlistenFns.push(
				await listen("instance-download-assets-started", () => {
					downloadState = "assets";
					isVisible = true;
				})
			);

			unlistenFns.push(
				await listen("instance-download-assets-finished", () => {
					downloadState = "done";
					isVisible = false;
				})
			);

			const assetTypes: ("assets" | "libraries" | "version-jar")[] = ["assets", "libraries", "version-jar"];

			for (const type of assetTypes) {
				const progressKey: keyof AssetsDownloadProgress = type === "version-jar" ? "versionJar" : type;

				unlistenFns.push(
					await listen<ProgressEvent>(`instance-download-${type}-progress`, (event) => {
						progress[progressKey] = event.payload.percentage;
						downloadState = type;
					})
				);
			}
		};

		setupEventListeners();

		return () => unlistenFns.forEach((unlisten) => unlisten());
	};

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
				<div {@attach downloadProgress} class="flex flex-col space-y-2">
					{@render ProgressBar(progress["assets"], "Assets")}
					{@render ProgressBar(progress["libraries"], "Libraries")}
					{@render ProgressBar(progress["version-jar"], "Minecraft")}
				</div>
			</Card.Content>
		</Card.Root>
	</div>
{/if}
