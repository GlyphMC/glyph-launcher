<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { platform } from "@tauri-apps/plugin-os";
	import { open } from "@tauri-apps/plugin-dialog";
	import { onMount } from "svelte";
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Input } from "$lib/components/ui/input";
	import type { ManualJava, ManualJavaTestResults, JavaConfig } from "$lib/types";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import { resetMode, setMode } from "mode-watcher";
	import * as Card from "$lib/components/ui/card";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import JavaDownloadPopUp from "$lib/components/core/JavaDownloadPopUp.svelte";
	import { saveJavaToConfig } from "$lib/utils";

	let isLinux = platform() === "linux";
	let useDiscreteGPU = $state(false);

	let showAutomaticJavaPopup = $state(false);

	let showManualJavaSetup = $state(false);
	let manualJava8 = $state<ManualJava>({ version: 8, path: "" });
	let manualJava17 = $state<ManualJava>({ version: 17, path: "" });
	let manualJava21 = $state<ManualJava>({ version: 21, path: "" });
	let showManualJavaTestPopup = $state(false);
	let manualJavaTestResults = $state<ManualJavaTestResults>();

	async function testJava() {
		await invoke<ManualJavaTestResults>("test_java", {
			paths: [manualJava8.path, manualJava17.path, manualJava21.path]
		}).then((data) => {
			let [java8, java17, java21] = data;
			showManualJavaTestPopup = true;
			manualJavaTestResults = [java8, java17, java21];
		});
	}

	onMount(async () => {
		await getJavaFromConfig();
	});

	async function getJavaFromConfig() {
		await invoke<JavaConfig>("get_java_from_config").then((data) => {
			let { java8Path, java17Path, java21Path } = data;
			manualJava8.path = java8Path;
			manualJava17.path = java17Path;
			manualJava21.path = java21Path;
		});
	}
</script>

{#if showAutomaticJavaPopup}
	<JavaDownloadPopUp onComplete={() => (showAutomaticJavaPopup = false)} />
{/if}

{#if showManualJavaTestPopup}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-900 bg-opacity-50 font-display backdrop-blur-md">
		<Card.Root class="relative w-full max-w-sm rounded-lg bg-zinc-900 p-2 text-center shadow-lg">
			<Card.Header>
				<h2 class="text-xl font-bold text-zinc-50">Test Java Setup</h2>
			</Card.Header>
			<Card.Content>
				{#if manualJavaTestResults}
					{#each manualJavaTestResults as result, index}
						<p class="text-zinc-50">
							Java {[8, 17, 21][index]}:
							{#if result}
								<span class="text-green-500">Success</span>
							{:else}
								<span class="text-red-500">Failed</span>
							{/if}
						</p>
					{/each}
				{/if}
			</Card.Content>
			<Card.Footer class="flex justify-center">
				<Button
					onclick={() => {
						showManualJavaTestPopup = false;
						saveJavaToConfig([manualJava8.path, manualJava17.path, manualJava21.path]);
					}}
					variant="outline">
					Save
				</Button>
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
			<Button onclick={() => (showAutomaticJavaPopup = true)}>Set up Java automatically</Button>
			<Button
				onclick={() => {
					showManualJavaSetup = true;
				}}
				variant="destructive">
				Set up Java manually
			</Button>
		</div>

		{#if showManualJavaSetup}
			{@render javaPathInput(manualJava8)}
			{@render javaPathInput(manualJava17)}
			{@render javaPathInput(manualJava21)}
			<Button class="mt-4" variant="secondary" onclick={testJava}>Test</Button>
		{/if}
	</div>
</div>

{#snippet javaPathInput(obj: { version: number; path: string })}
	<div class="mt-4">
		<Label class="mb-2 block text-zinc-50">Java {obj.version} Path</Label>
		<div class="flex flex-row">
			<Input type="text" placeholder="/usr/lib/jvm" bind:value={obj.path} />
			<Button
				class="ml-2 h-8"
				variant="outline"
				onclick={async (e) => {
					e.preventDefault();

					let isWindows = platform() === "windows";
					let javaPath = await open({
						title: "Select Java",
						multiple: false,
						filters: isWindows ? [{ name: "Java", extensions: ["exe"] }] : []
					});

					if (javaPath) {
						obj.path = javaPath;
					}
				}}>
				...
			</Button>
		</div>
	</div>
{/snippet}
