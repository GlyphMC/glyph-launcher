<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import { ProgressBar } from "./ProgressBar.svelte";
	import type { JavaDownloadState, JavaExtractState, ProgressEvent, JavaProgress, JavaPaths, DownloadPaths } from "$lib/types";
	import { saveJavaToConfig } from "$lib/utils/JavaUtils";
	import { onMount } from "svelte";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { invoke } from "@tauri-apps/api/core";
	import type { Attachment } from "svelte/attachments";
	import { scale } from "svelte/transition";
	import { quintOut } from "svelte/easing";

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
			unlistenFns.push(
				await listen("java-download-started", () => {
					downloadState = "downloading";
					console.log("Java download started");
				})
			);
			unlistenFns.push(
				await listen<DownloadPaths>("java-download-finished", async (event) => {
					downloadState = "done";
					paths = event.payload.paths;
					await startExtraction();
				})
			);

			unlistenFns.push(await listen("java-extract-started", () => (extractState = "extracting")));
			unlistenFns.push(
				await listen("java-extract-finished", async () => {
					extractState = "done";
					await saveJavaToConfig(paths, true);
				})
			);

			[8, 17, 21].forEach((version) => {
				setProgressListener("java-download", "download", version);
				setProgressListener("java-extract", "extract", version);
			});
		};

		setupEventListeners();

		return () => unlistenFns.forEach((unlisten) => unlisten());
	};

	function setProgressListener(name: string, stateKey: keyof typeof javaProgress, version: number) {
		listen<ProgressEvent>(`${name}-progress-${version}`, (event) => {
			const updatedVersionMap = {
				...javaProgress[stateKey],
				[version]: event.payload.percentage
			};
			javaProgress[stateKey] = updatedVersionMap;
		});
	}

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
		await invoke<JavaPaths>("download_java").then((data) => {
			paths = data.map((path) => path.replace(".zip", ""));
			console.log("Java downloaded successfully");
		});
	}

	async function startExtraction() {
		downloadState = "none";
		await invoke<JavaPaths>("extract_java", { paths }).then((data) => {
			console.log(data);
			paths = data;
			console.log("Java extracted successfully");
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
