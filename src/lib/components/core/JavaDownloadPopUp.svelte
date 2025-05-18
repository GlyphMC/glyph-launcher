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

	let statusText = $derived(getStatusText(downloadState, extractState));
	let isButtonDisabled = $derived(downloadState === "downloading" || extractState === "extracting");
	let currentProgress = $derived(getCurrentProgress());

	onMount(async () => await downloadJava());

	const javaSetup: Attachment = () => {
		let unlistenFns: UnlistenFn[] = [];

		const setupEventListeners = async () => {
			unlistenFns.push(await events.javaDownloadStartedEvent.listen((event) => (downloadState = "downloading")));
			unlistenFns.push(
				await events.javaDownloadProgressEvent.listen((event) => {
					const { version, percentage } = event.payload;

					const updatedVersionMap = {
						...javaProgress.download,
						[version]: percentage
					};
					javaProgress.download = updatedVersionMap;
				})
			);
			unlistenFns.push(
				await events.javaDownloadFinishedEvent.listen(async (event) => {
					downloadState = "done";
					paths = event.payload.paths;
					await startExtraction();
				})
			);

			unlistenFns.push(await events.javaExtractStartedEvent.listen((event) => (extractState = "extracting")));
			unlistenFns.push(
				await events.javaExtractProgressEvent.listen((event) => {
					const { version, percentage } = event.payload;

					const updatedVersionMap = {
						...javaProgress.extract,
						[version]: percentage
					};
					javaProgress.extract = updatedVersionMap;
				})
			);
			unlistenFns.push(
				await events.javaExtractFinishedEvent.listen(async (event) => {
					extractState = "done";
					await saveJavaToConfig(event.payload.paths, true);
				})
			);
		};

		setupEventListeners();

		return () => unlistenFns.forEach((unlisten) => unlisten());
	};

	function getStatusText(download: JavaDownloadState, extract: JavaExtractState): string {
		if (download === "downloading") {
			return "Downloading:";
		} else if (download === "done") {
			return "Finished downloading. Ready to extract";
		} else if (extract === "extracting") {
			return "Extracting:";
		} else if (extract === "done") {
			return "Finished extracting.";
		}

		return "";
	}

	function getCurrentProgress() {
		return downloadState === "downloading" || downloadState === "done" ? javaProgress.download : javaProgress.extract;
	}

	async function downloadJava() {
		await commands.downloadJava().then((res) => {
			if (res.status === "ok") {
				paths = res.data.map((path) => path.replace(".zip", ""));
				console.log("Java downloaded successfully");
			} else {
				console.error("Java download failed:", res.error);
			}
		});
	}

	async function startExtraction() {
		downloadState = "none";
		if (!paths || paths.length !== 3) return;

		const pathsForExtraction: [string, string, string] = [paths[0], paths[1], paths[2]];
		await commands.extractJava(pathsForExtraction).then((res) => {
			if (res.status === "ok") {
				paths = res.data.map((path) => path.replace(".zip", ""));
				console.log("Java extracted successfully");
			} else {
				console.error("Java extraction failed:", res.error);
			}
		});
	}

	type Props = {
		onComplete: () => void;
	};

	let { onComplete }: Props = $props();
</script>

<div
	class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md"
	in:scale={{ duration: 300, start: 0.5, opacity: 0, easing: quintOut }}
	out:scale={{ duration: 200, opacity: 0, easing: quintOut }}
	{@attach javaSetup}>
	<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
		<Card.Header>
			<h2 class="text-xl font-bold text-zinc-50">Java Automatic Setup</h2>
			<p class="text-sm text-zinc-50">{statusText}</p>
		</Card.Header>

		<Card.Content>
			<div class="flex flex-col space-y-2">
				{@render ProgressBar(currentProgress[8], "Java 8")}
				{@render ProgressBar(currentProgress[17], "Java 17")}
				{@render ProgressBar(currentProgress[21], "Java 21")}
			</div>
		</Card.Content>

		<Card.Footer class="flex justify-center">
			<Button onclick={() => onComplete()} variant="outline" disabled={isButtonDisabled}>Done</Button>
		</Card.Footer>
	</Card.Root>
</div>
