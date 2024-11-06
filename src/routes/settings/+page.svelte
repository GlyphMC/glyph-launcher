<script lang="ts">
	import type { ExtractState, DownloadState, JavaDownloadPaths, Payload, Progress } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { platform } from "@tauri-apps/plugin-os";
	import { onMount } from "svelte";
	import ProgressBars from "$lib/components/ProgressBars.svelte";

	let windowWidth = $state(854);
	let windowHeight = $state(480);
	let isLinux = $state(false);
	let isDiscreteGPU = $state(false);

	let downloadState = $state<DownloadState>("none");
	let extractState = $state<ExtractState>("none");
	let paths = $state<string[]>([]);
	let java8DownloadProgress = $state(0);
	let java17DownloadProgress = $state(0);
	let java21DownloadProgress = $state(0);
	let java8ExtractProgress = $state(0);
	let java17ExtractProgress = $state(0);
	let java21ExtractProgress = $state(0);

	onMount(() => {
		let platformName = platform();
		isLinux = platformName === "linux";
		downloadState = "none";
		extractState = "none";
	});

	async function downloadJava() {
		invoke<JavaDownloadPaths>("download_java").then((data) => {
			paths = data;
			console.log("Java downloaded successfully");
		});
	}

	async function extractJava() {
		invoke("extract_java", { java8ArchivePath: paths[0], java17ArchivePath: paths[1], java21ArchivePath: paths[2] }).then(() => {
			console.log("Java extracted successfully");
		});
	}

	listen<Payload>("download-started", () => {
		downloadState = "downloading";
	});

	listen<Payload>("download-finished", () => {
		downloadState = "done";
	});

	listen<Payload>("extract-started", () => {
		extractState = "extracting";
	});

	listen<Payload>("extract-finished", () => {
		extractState = "done";
	});

	listen<Progress>("java-download-progress-8", (event) => {
		java8DownloadProgress = event.payload.percentage;
	});

	listen<Progress>("java-download-progress-17", (event) => {
		java17DownloadProgress = event.payload.percentage;
	});

	listen<Progress>("java-download-progress-21", (event) => {
		java21DownloadProgress = event.payload.percentage;
	});

	listen<Progress>("java-extract-progress-8", (event) => {
		java8ExtractProgress = event.payload.percentage;
	});

	listen<Progress>("java-extract-progress-17", (event) => {
		java17ExtractProgress = event.payload.percentage;
	});

	listen<Progress>("java-extract-progress-21", (event) => {
		java21ExtractProgress = event.payload.percentage;
	});
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
				<ProgressBars java8Progress={java8DownloadProgress} java17Progress={java17DownloadProgress} java21Progress={java21DownloadProgress} />
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
				<ProgressBars java8Progress={java8ExtractProgress} java17Progress={java17ExtractProgress} java21Progress={java21ExtractProgress} />
				<button
					onclick={() => {
						downloadState = "none";
						extractState = "none";
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
				class="rounded bg-green-600 px-4 py-2 font-bold text-zinc-50 transition duration-75 ease-in-out active:scale-105 hover:bg-green-700">
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
