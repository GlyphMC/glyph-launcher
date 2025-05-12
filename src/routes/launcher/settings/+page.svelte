<script lang="ts">
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import JavaDownloadPopUp from "$lib/components/core/JavaDownloadPopUp.svelte";
	import ManualJavaSetup from "$lib/components/core/ManualJavaSetup.svelte";
	import type { LauncherSettings } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import { resetMode, setMode } from "mode-watcher";

	let discordRichPresence = $state(true);
	let useDiscreteGpu = $state(true);

	let showAutomaticJavaPopup = $state(false);
	let showManualJavaSetup = $state(false);

	let initialSettings: LauncherSettings | null = $state(null);

	async function loadSettings() {
		try {
			await invoke<LauncherSettings>("get_launcher_settings").then((data) => {
				initialSettings = data;
				discordRichPresence = data.richPresence;
				useDiscreteGpu = data.useDiscreteGpu;

				console.log("Settings loaded", $state.snapshot(initialSettings));
			});
		} catch (error) {
			console.error("Error loading settings:", error);
		}
	}

	onMount(async () => await loadSettings());

	async function handleSubmit() {
		const settingsToSave: LauncherSettings = {
			richPresence: discordRichPresence,
			useDiscreteGpu: useDiscreteGpu
		};

		try {
			await invoke("save_launcher_settings", { settings: settingsToSave });
			initialSettings = { ...settingsToSave };
			console.log("Settings saved", $state.snapshot(initialSettings));
		} catch (error) {
			console.error("Error saving settings:", error);
		}
	}
</script>

{#if showAutomaticJavaPopup}
	<JavaDownloadPopUp onComplete={() => (showAutomaticJavaPopup = false)} />
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
		<div class="mt-4">
			<div class="inline-flex items-center">
				<Checkbox id="discordRichPresence" bind:checked={discordRichPresence} />
				<Label for="discordRichPresence" class="ml-2 text-zinc-50">Discord Rich Presence</Label>
			</div>
		</div>
	</div>

	<div class="px-10 py-5">
		<h2 class="text-2xl font-semibold text-zinc-50">Minecraft</h2>
		<div class="mt-4">
			<div class="mt-4">
				<div class="inline-flex items-center">
					<Checkbox id="discreteGpu" bind:checked={useDiscreteGpu} />
					<Label id="discreteGpu" class="ml-2 text-zinc-50">Use discrete GPU</Label>
				</div>
				<!-- <div class="mt-4">
					<div class="inline-flex items-center">
						<Checkbox id="autoOpenLogs" bind:checked={autoOpenLogs} />
						<Label for="autoOpenLogs" class="ml-2 text-zinc-50">Auto open logs</Label>
					</div>
				</div> -->
			</div>
		</div>
	</div>

	<div class="px-10 py-5">
		<h2 class="text-2xl font-semibold text-zinc-50">Java</h2>
		<div class="mt-4 flex space-x-4">
			<Button onclick={() => (showAutomaticJavaPopup = true)}>Set up Java automatically</Button>
			<Button onclick={() => (showManualJavaSetup = true)} variant="destructive">Set up Java manually</Button>
		</div>

		{#if showManualJavaSetup}
			<ManualJavaSetup />
		{/if}

		<div class="py-8">
			<Button type="submit" onclick={handleSubmit}>Save Settings</Button>
		</div>
	</div>
</div>
