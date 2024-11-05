<script lang="ts">
	import type { DownloadState, JavaDownloadPaths, Payload, Progress } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { platform } from "@tauri-apps/plugin-os";
	import { onMount } from "svelte";

	let windowWidth = $state(854);
	let windowHeight = $state(480);
	let isLinux = $state(false);
	let isDiscreteGPU = $state(false);

	let downloadState = $state<DownloadState>("none");
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
	});

	async function downloadJava() {
		console.log("Setting up Java automatically");

		invoke<JavaDownloadPaths>("download_java").then((data) => {
			paths = data;
			console.log("Java downloaded successfully");
		});
	}

	async function extractJava() {
		console.log("Extracting Java");

		invoke("extract_java", {  }).then(() => {
			console.log("Java extracted successfully");
		});
	}

	listen<Payload>("download-started", () => {
		downloadState = "downloading";
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
</script>

{#snippet ProgressBar(percentage: number)}
	<div class="h-3 rounded-full bg-zinc-200">
		<div class="h-full rounded-full bg-green-600" style="width: {percentage}%"></div>
	</div>
{/snippet}

{#if downloadState !== "none"}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md">
		<div class="relative w-full max-w-sm rounded-lg bg-zinc-700 p-6 text-center shadow-lg">
			<h2 class="text-xl font-bold text-zinc-50">Java Automatic Setup</h2>
			{#if downloadState === "downloading"}
				<p class="text-lg text-zinc-50">Downloading:</p>
			{/if}
			<div class="flex items-center">
				<p class="mr-4 text-zinc-50">Java 8</p>
				<div class="flex-1">
					{@render ProgressBar(java8DownloadProgress)}
				</div>
			</div>
			<div class="flex items-center">
				<p class="mr-4 text-zinc-50">Java 17</p>
				<div class="flex-1">
					{@render ProgressBar(java17DownloadProgress)}
				</div>
			</div>
			<div class="flex items-center">
				<p class="mr-4 text-zinc-50">Java 21</p>
				<div class="flex-1">
					{@render ProgressBar(java21DownloadProgress)}
				</div>
			</div>
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
				class="rounded bg-green-600 px-4 py-2 font-bold text-zinc-50 transition duration-75 ease-in-out active:scale-105">
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
