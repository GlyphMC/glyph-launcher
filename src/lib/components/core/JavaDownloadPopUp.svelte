<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import { ProgressBar } from "./ProgressBar.svelte";
	import { saveJavaToConfig, type JavaDownloadState, type JavaExtractState, type JavaProgress } from "$lib/utils/JavaUtils";
	import { onMount } from "svelte";
	import { type UnlistenFn } from "@tauri-apps/api/event";
	import type { Attachment } from "svelte/attachments";
	import { scale } from "svelte/transition";
	import { quintOut } from "svelte/easing";
	import { commands, events } from "$lib/bindings";

	let downloadState = $state<JavaDownloadState>("none");
	let extractState = $state<JavaExtractState>("none");
	let paths = $state<string[]>([]);
	let javaProgress = $state<JavaProgress>({
		download: { 8: 0, 17: 0, 21: 0 },
		extract: { 8: 0, 17: 0, 21: 0 }
	});

	let statusText = $derived(getStatusText());
	let isButtonDisabled = $derived(downloadState === "downloading" || extractState === "extracting");
	let currentProgress = $derived(getCurrentProgress());

	onMount(async () => {
		if (missingVersions && missingVersions.length > 0) {
			await downloadJava();
		}
	});

	const javaSetup: Attachment = () => {
		let unlistenFns: UnlistenFn[] = [];

		const setupEventListeners = async () => {
			unlistenFns.push(await events.javaDownloadStartedEvent.listen((event) => (downloadState = "downloading")));
			unlistenFns.push(
				await events.javaDownloadProgressEvent.listen((event) => {
					const { version, percentage } = event.payload;
					javaProgress.download = { ...javaProgress.download, [version]: percentage };
				})
			);
			unlistenFns.push(
				await events.javaDownloadFinishedEvent.listen(async (event) => {
					downloadState = "done";
					paths = event.payload.paths;
					await extractJava();
				})
			);

			unlistenFns.push(await events.javaExtractStartedEvent.listen((event) => (extractState = "extracting")));
			unlistenFns.push(
				await events.javaExtractProgressEvent.listen((event) => {
					const { version, percentage } = event.payload;
					javaProgress.extract = { ...javaProgress.extract, [version]: percentage };
				})
			);
			unlistenFns.push(
				await events.javaExtractFinishedEvent.listen(async (event) => {
					extractState = "done";
					paths = event.payload.paths;
					await completeSetup();
				})
			);
		};

		setupEventListeners();
		return () => unlistenFns.forEach((unlisten) => unlisten());
	};

	function getStatusText(): string {
		if (downloadState === "downloading") return "Downloading missing Java versions...";
		if (downloadState === "done") return "Download complete. Starting extraction...";
		if (extractState === "extracting") return "Extracting Java installations...";
		if (extractState === "done") return "Setup complete! Java installations ready.";
		return "";
	}

	function getCurrentProgress() {
		return downloadState === "downloading" || downloadState === "done" ? javaProgress.download : javaProgress.extract;
	}

	async function downloadJava() {
		if (!missingVersions || missingVersions.length === 0) {
			console.log("No missing Java versions to download");
			return;
		}

		console.log("Downloading Java versions:", missingVersions);

		await commands.downloadJava(missingVersions).then((res) => {
			if (res.status === "ok") {
				paths = res.data.map((path) => path.replace(".zip", ""));
				console.log("Java downloaded successfully");
			} else {
				console.error("Java download failed:", res.error);
			}
		});
	}

	async function extractJava() {
		downloadState = "none";
		if (!paths || paths.length === 0 || !missingVersions || missingVersions.length === 0) {
			console.error("No paths or missing versions available for extraction");
			return;
		}

		await commands.extractJava(paths, missingVersions).then((res) => {
			if (res.status === "ok") {
				paths = res.data.map((path) => path.replace(".zip", ""));
				console.log("Java extracted successfully");
			} else {
				console.error("Java extraction failed:", res.error);
			}
		});
	}

	async function completeSetup() {
		const finalPaths: [string, string, string] = [detectedJava?.java8 || "", detectedJava?.java17 || "", detectedJava?.java21 || ""];

		if (missingVersions && paths.length > 0) {
			let pathIndex = 0;
			missingVersions.forEach((version) => {
				if (pathIndex < paths.length) {
					const arrayIndex = version === 8 ? 0 : version === 17 ? 1 : 2;
					if (paths[pathIndex] && paths[pathIndex].trim() !== "") {
						const javaExePath = paths[pathIndex].endsWith("bin") ? `${paths[pathIndex]}/javaw.exe` : `${paths[pathIndex]}/bin/javaw.exe`;
						finalPaths[arrayIndex] = javaExePath;
					}
					pathIndex++;
				}
			});
		}

		await saveJavaToConfig(finalPaths, true);
		onComplete(finalPaths);
	}

	type Props = {
		onComplete: (finalPaths: [string, string, string]) => void;
		missingVersions?: number[];
		detectedJava?: {
			java8: string | null;
			java17: string | null;
			java21: string | null;
		};
	};

	let { onComplete, missingVersions, detectedJava }: Props = $props();
</script>

<div
	class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md"
	in:scale={{ duration: 300, start: 0.5, opacity: 0, easing: quintOut }}
	out:scale={{ duration: 200, opacity: 0, easing: quintOut }}
	{@attach javaSetup}>
	<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
		<Card.Header>
			<h2 class="text-xl font-bold text-zinc-50">Java Automatic Setup</h2>
			{#if missingVersions?.length}
				<p class="text-sm text-zinc-300">Installing: Java {missingVersions.join(", ")}</p>
			{/if}
			<p class="text-sm text-zinc-50">{statusText}</p>
		</Card.Header>

		<Card.Content>
			<div class="flex flex-col space-y-2">
				{#each missingVersions || [] as version}
					{@render ProgressBar(currentProgress[version], `Java ${version}`)}
				{/each}
			</div>
		</Card.Content>

		<Card.Footer class="flex justify-center">
			<Button onclick={() => completeSetup()} variant="outline" disabled={isButtonDisabled}>
				{extractState === "done" ? "Finish" : "Please Wait..."}
			</Button>
		</Card.Footer>
	</Card.Root>
</div>
