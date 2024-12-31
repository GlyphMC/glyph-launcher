<script lang="ts">
	import * as Card from "$lib/components/ui/card";
	import { Button } from "$lib/components/ui/button";
	import ProgressBars from "$lib/components/core/ProgressBars.svelte";
	import type { DownloadState, ExtractState, ProgressEvent, JavaProgress, JavaPaths, DownloadPaths } from "$lib/types";
	import { saveJavaToConfig } from "$lib/utils";
	import { onMount } from "svelte";
	import { listen } from "@tauri-apps/api/event";
	import { invoke } from "@tauri-apps/api/core";

	let downloadState = $state<DownloadState>("none");
	let extractState = $state<ExtractState>("none");
	let paths = $state<string[]>([]);
	let javaProgress = $state<JavaProgress>({
		download: { 8: 0, 17: 0, 21: 0 },
		extract: { 8: 0, 17: 0, 21: 0 }
	});

	let statusText = $derived(getStatusText(downloadState, extractState));
	let buttonText = $derived(getButtonText());
	let isButtonDisabled = $derived(downloadState === "downloading" || extractState === "extracting");
	let currentProgress = $derived(getCurrentProgress());

	onMount(async () => {
		setupEventListeners();
		await downloadJava();
	});

	function setupEventListeners() {
		listen("download-started", () => (downloadState = "downloading"));
		listen<DownloadPaths>("download-finished", (event) => {
			downloadState = "done";
			event.payload.paths = paths;
		});
		listen("extract-started", () => (extractState = "extracting"));
		listen("extract-finished", () => (extractState = "done"));

		[8, 17, 21].forEach((version) => {
			setProgressListener("java-download", "download", version);
			setProgressListener("java-extract", "extract", version);
		});
	}

	function setProgressListener(name: string, stateKey: keyof typeof javaProgress, version: number) {
		listen<ProgressEvent>(`${name}-progress-${version}`, (event) => {
			javaProgress[stateKey][version] = event.payload.percentage;
		});
	}

	function getStatusText(download: DownloadState, extract: ExtractState): string {
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

	function getButtonText() {
		return downloadState === "downloading" || downloadState === "done" ? "Extract" : "Done";
	}

	function getCurrentProgress() {
		return downloadState === "downloading" || downloadState === "done" ? javaProgress.download : javaProgress.extract;
	}

	async function downloadJava() {
		await invoke<JavaPaths>("download_java").then((data) => {
			paths = data;
			console.log("Java downloaded successfully");
		});
	}

	async function extractJava() {
		await invoke<JavaPaths>("extract_java", { paths }).then((data) => {
			console.log(data);
			paths = data;
			console.log("Java extracted successfully");
		});
	}

	async function handleAction() {
		if (downloadState === "done") {
			downloadState = "none"
			await extractJava();
			return;
		}

		if (extractState === "done") {
			await saveJavaToConfig(paths);
			onComplete();
			return;
		}
	}

	type Props = {
		onComplete: () => void;
	};

	let { onComplete }: Props = $props();
</script>


<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md">
	<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
		<Card.Header>
			<h2 class="text-xl font-bold text-zinc-50">Java Automatic Setup</h2>
			<p class="text-sm text-zinc-50">{statusText}</p>
		</Card.Header>

		<Card.Content>
			<ProgressBars java8Progress={currentProgress[8]} java17Progress={currentProgress[17]} java21Progress={currentProgress[21]} />
		</Card.Content>

		<Card.Footer class="flex justify-center">
			<Button onclick={handleAction} variant="outline" disabled={isButtonDisabled}>
				{buttonText}
			</Button>
		</Card.Footer>
	</Card.Root>
</div>
