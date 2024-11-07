<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { platform } from "@tauri-apps/plugin-os";
	import { onMount } from "svelte";
	import ProgressBars from "$lib/components/ProgressBars.svelte";
	import type { ExtractState, DownloadState, JavaPaths, ProgressEvent, JavaProgress } from "$lib/types";

	let windowWidth = $state(854);
	let windowHeight = $state(480);
	let isLinux = $state(false);
	let isDiscreteGPU = $state(false);

	let downloadState = $state<DownloadState>("none");
	let extractState = $state<ExtractState>("none");
	let paths = $state<string[]>([]);
	let javaProgress = $state<JavaProgress>({
		download: { 8: 0, 17: 0, 21: 0 },
		extract: { 8: 0, 17: 0, 21: 0 }
	});

	onMount(() => {
		let platformName = platform();
		isLinux = platformName === "linux";
		resetStates();
	});

	function resetStates() {
		downloadState = "none";
		extractState = "none";
	}

	async function downloadJava() {
		invoke<JavaPaths>("download_java").then((data) => {
			paths = data;
			console.log("Java downloaded successfully");
		});
	}

	async function extractJava() {
		invoke<JavaPaths>("extract_java", { paths }).then((data) => {
			console.log(data);
			paths = data;
			console.log("Java extracted successfully");
		});
	}

	async function saveJavaToConfig() {
		invoke("save_java_to_config", { paths }).then(() => {
			console.log("Java saved to config successfully");
		});
	}

	listen("download-started", () => (downloadState = "downloading"));
	listen("download-finished", () => (downloadState = "done"));
	listen("extract-started", () => (extractState = "extracting"));
	listen("extract-finished", () => (extractState = "done"));

	function setProgressListener(name: String, stateKey: keyof typeof javaProgress, version: number) {
		listen<ProgressEvent>(`${name}-progress-${version}`, (event) => {
			javaProgress[stateKey][version] = event.payload.percentage;
		});
	}

	[8, 17, 21].forEach((version) => {
		setProgressListener("java-download", "download", version);
		setProgressListener("java-extract", "extract", version);
	});
	//TODO: tailwind motion
</script>

{#if downloadState !== "none" || extractState !== "none"}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md">
		<div class="relative w-full max-w-sm rounded-lg bg-zinc-700 p-6 text-center shadow-lg">
			<h2 class="text-xl font-bold text-zinc-50">Java Automatic Setup</h2>
			{#if downloadState === "downloading"}
				<p class="text-lg text-zinc-50">Downloading:</p>
			{:else if downloadState === "done"}
				<p class="text-lg text-zinc-50">Finished downloading. Ready to extract</p>
			{:else if extractState === "extracting"}
				<p class="text-lg text-zinc-50">Extracting:</p>
			{:else if extractState === "done"}
				<p class="text-lg text-zinc-50">Finished extracting.</p>
			{/if}

			{#if downloadState === "downloading" || downloadState === "done"}
				<ProgressBars
					java8Progress={javaProgress.download[8]}
					java17Progress={javaProgress.download[17]}
					java21Progress={javaProgress.download[21]} />
				<button
					onclick={() => {
						extractJava();
						downloadState = "none";
						extractState = "extracting";
					}}
					class="mt-4 rounded-md bg-green-600 px-8 py-1.5 font-bold text-zinc-50 transition duration-75 ease-in-out hover:bg-green-700 active:scale-105 disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-green-600"
					disabled={downloadState === "downloading"}>
					Extract
				</button>
			{:else if extractState === "extracting" || extractState === "done"}
				<ProgressBars
					java8Progress={javaProgress.extract[8]}
					java17Progress={javaProgress.extract[17]}
					java21Progress={javaProgress.extract[21]} />
				<button
					onclick={() => {
						resetStates();
						saveJavaToConfig();
					}}
					class="mt-4 rounded-md bg-green-600 px-8 py-1.5 font-bold text-zinc-50 transition duration-75 ease-in-out hover:bg-green-700 active:scale-105 disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-green-600"
					disabled={extractState === "extracting"}>
					Done
				</button>
			{/if}
		</div>
	</div>
{/if}

<div class="overflow-hidden bg-zinc-400 font-display">
	<p class="px-10 pt-10 text-3xl font-bold text-zinc-50">Settings</p>

	<div class="px-10 py-5">
		<h2 class="text-2xl font-semibold text-zinc-50">Minecraft</h2>
		<div class="mt-4">
			<label class="inline-flex items-center">
				<input type="checkbox" class="form-checkbox text-zinc-600" />
				<span class="ml-2 text-zinc-50">Start maximized</span>
			</label>
			<div class="mt-4">
				<label for="window-width" class="block text-zinc-50">Window Width</label>
				<input
					id="window-width"
					type="number"
					class="form-input mt-1 block w-full text-zinc-900"
					placeholder="Enter width"
					bind:value={windowWidth} />
			</div>
			<div class="mt-4">
				<label for="window-height" class="block text-zinc-50">Window Height</label>
				<input
					id="window-height"
					type="number"
					class="form-input mt-1 block w-full text-zinc-900"
					placeholder="Enter height"
					bind:value={windowHeight} />
			</div>
			{#if isLinux}
				<div class="mt-4">
					<label class="inline-flex items-center">
						<input type="checkbox" class="form-checkbox text-zinc-600" />
						<span class="ml-2 text-zinc-50">Use discrete GPU</span>
					</label>
				</div>
			{/if}
		</div>
	</div>

	<div class="px-10 py-5">
		<h2 class="text-2xl font-semibold text-zinc-50">Java</h2>
		<div class="mt-4">
			<button
				onclick={downloadJava}
				class="rounded bg-green-600 px-4 py-2 font-bold text-zinc-50 transition duration-75 ease-in-out hover:bg-green-700 active:scale-105">
				Set up Java automatically
			</button>
		</div>
		<!-- <div class="mt-4">
			<label for="java-path" class="block text-zinc-50">Java Path</label>
			<input id="java-path" type="text" class="form-input mt-1 block w-full text-zinc-900" placeholder="Enter Java path" />
		</div>
		<div class="mt-4">
			<label for="java-args" class="block text-zinc-50">Java Arguments</label>
			<input id="java-args" type="text" class="form-input mt-1 block w-full text-zinc-900" placeholder="Enter Java arguments" />
		</div> -->
	</div>
</div>
