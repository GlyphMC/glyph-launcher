<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { listen } from "@tauri-apps/api/event";
	import { platform } from "@tauri-apps/plugin-os";
	import { onMount } from "svelte";
	import ProgressBars from "$lib/components/core/ProgressBars.svelte";
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Input } from "$lib/components/ui/input";
	import type { ExtractState, DownloadState, JavaPaths, ProgressEvent, JavaProgress } from "$lib/types";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import { resetMode, setMode } from "mode-watcher";
	import * as Card from "$lib/components/ui/card";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

	let startMaximized = $state(false);
	let windowWidth = $state(854);
	let windowHeight = $state(480);
	let isLinux = $state(false);
	let useDiscreteGPU = $state(false);

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

	function setProgressListener(name: string, stateKey: keyof typeof javaProgress, version: number) {
		listen<ProgressEvent>(`${name}-progress-${version}`, (event) => {
			javaProgress[stateKey][version] = event.payload.percentage;
		});
	}

	[8, 17, 21].forEach((version) => {
		setProgressListener("java-download", "download", version);
		setProgressListener("java-extract", "extract", version);
	});
</script>

{#if downloadState !== "none" || extractState !== "none"}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md">
		<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
			<Card.Header>
				<h2 class="text-xl font-bold text-zinc-50">Java Automatic Setup</h2>
				{#if downloadState === "downloading"}
					<p class="text-sm text-zinc-50">Downloading:</p>
				{:else if downloadState === "done"}
					<p class="text-sm text-zinc-50">Finished downloading. Ready to extract</p>
				{:else if extractState === "extracting"}
					<p class="text-sm text-zinc-50">Extracting:</p>
				{:else if extractState === "done"}
					<p class="text-sm text-zinc-50">Finished extracting.</p>
				{/if}
			</Card.Header>

			<Card.Content>
				{#if downloadState === "downloading" || downloadState === "done"}
					<ProgressBars
						java8Progress={javaProgress.download[8]}
						java17Progress={javaProgress.download[17]}
						java21Progress={javaProgress.download[21]} />
				{:else if extractState === "extracting" || extractState === "done"}
					<ProgressBars
						java8Progress={javaProgress.extract[8]}
						java17Progress={javaProgress.extract[17]}
						java21Progress={javaProgress.extract[21]} />
				{/if}
			</Card.Content>

			<Card.Footer class="flex justify-center">
				{#if downloadState === "downloading" || downloadState === "done"}
					<Button
						onclick={() => {
							extractJava();
							downloadState = "none";
							extractState = "extracting";
						}}
						variant="outline"
						disabled={downloadState === "downloading"}>
						Extract
					</Button>
				{:else if extractState === "extracting" || extractState === "done"}
					<Button
						onclick={() => {
							resetStates();
							saveJavaToConfig();
						}}
						variant="outline"
						disabled={extractState === "extracting"}>
						Done
					</Button>
				{/if}
			</Card.Footer>
		</Card.Root>
	</div>
{/if}

<div class="w-full overflow-hidden font-display">
	<p class="px-10 pt-10 text-3xl font-bold text-zinc-50">Settings</p>

	<div class="px-10 py-5">
		<h2 class="mb-4 text-2xl font-semibold text-zinc-50">General</h2>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger class={buttonVariants({ variant: "outline", size: "default" })}>Theme</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end">
				<DropdownMenu.Item onclick={() => setMode("light")}>Light</DropdownMenu.Item>
				<DropdownMenu.Item onclick={() => setMode("dark")}>Dark</DropdownMenu.Item>
				<DropdownMenu.Item onclick={() => resetMode()}>System</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>

	<div class="px-10 py-5">
		<h2 class="text-2xl font-semibold text-zinc-50">Minecraft</h2>
		<div class="mt-4">
			<div class="inline-flex items-center">
				<Checkbox bind:checked={startMaximized} />
				<Label class="ml-2 text-zinc-50">Start maximised</Label>
			</div>

			<div class="mt-4 flex space-x-4">
				<div>
					<Label class="mb-2 block text-zinc-50">Window Width</Label>
					<Input type="number" bind:value={windowWidth} />
				</div>
				<div>
					<Label class="mb-2 block text-zinc-50">Window Height</Label>
					<Input type="number" bind:value={windowHeight} />
				</div>
			</div>
			{#if isLinux}
				<div class="mt-4">
					<div class="inline-flex items-center">
						<Checkbox bind:checked={useDiscreteGPU} />
						<Label class="ml-2 text-zinc-50">Use discrete GPU</Label>
					</div>
				</div>
			{/if}
		</div>
	</div>

	<div class="px-10 py-5">
		<h2 class="text-2xl font-semibold text-zinc-50">Java</h2>
		<div class="mt-4 flex space-x-4">
			<Button onclick={downloadJava}>Set up Java automatically</Button>
			<Button
				onclick={() => {
					resetStates();
				}}
				variant="destructive">
				Set up Java manually
			</Button>
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
