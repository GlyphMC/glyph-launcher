<script lang="ts">
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import * as RadioGroup from "$lib/components/ui/radio-group";
	import JavaDownloadPopUp from "$lib/components/core/JavaDownloadPopUp.svelte";
	import ManualJavaSetup from "$lib/components/core/ManualJavaSetup.svelte";
	import { onMount } from "svelte";
	import { mode, resetMode, setMode } from "mode-watcher";
	import { commands } from "$lib/bindings";
	import { Monitor, Moon, Sun } from "@lucide/svelte";

	type LauncherSettings = {
		richPresence: boolean;
		useDiscreteGpu: boolean;
	};

	let discordRichPresence = $state(true);
	let useDiscreteGpu = $state(true);
	let showAutomaticJavaPopup = $state(false);
	let showManualJavaSetup = $state(false);
	let initialSettings: LauncherSettings | null = $state(null);
	let selectedTheme = $state("system");

	async function loadSettings() {
		await commands.getLauncherSettings().then((res) => {
			if (res.status === "ok") {
				initialSettings = res.data;
				discordRichPresence = res.data.richPresence;
				useDiscreteGpu = res.data.useDiscreteGpu;

				console.log("Settings loaded", $state.snapshot(initialSettings));
			} else {
				console.error("Failed to get launcher settings:", res.error);
			}
		});
	}

	function mapModeToTheme(currentMode: string): string {
		switch (currentMode) {
			case "light":
				return "light";
			case "dark":
				return "dark";
			default:
				return "system";
		}
	}

	function handleThemeChange(value: string) {
		console.log("Theme changed to:", value);
		selectedTheme = value;

		switch (value) {
			case "light":
				setMode("light");
				break;
			case "dark":
				setMode("dark");
				break;
			case "system":
				resetMode();
				break;
		}
	}

	async function handleSubmit() {
		const settingsToSave: LauncherSettings = {
			richPresence: discordRichPresence,
			useDiscreteGpu: useDiscreteGpu
		};

		await commands.saveLauncherSettings(settingsToSave).then((res) => {
			if (res.status === "ok") {
				console.log("Settings saved", $state.snapshot(initialSettings));
			} else {
				console.error("Failed to save settings:", res.error);
			}
		});
	}

	onMount(async () => {
		await loadSettings();
		selectedTheme = mapModeToTheme(mode.current || "system");
	});

	$effect(() => {
		selectedTheme = mapModeToTheme(mode.current || "system");
	});
</script>

{#if showAutomaticJavaPopup}
	<JavaDownloadPopUp onComplete={() => (showAutomaticJavaPopup = false)} />
{/if}

<div class="w-full overflow-hidden font-display">
	<p class="px-10 pt-10 text-3xl font-bold text-zinc-50">Settings</p>
	<div class="space-y-8 px-10 py-5">
		<div>
			<h2 class="mb-4 text-2xl font-semibold text-zinc-50">General</h2>
			<div class="space-y-6">
				<div class="space-y-3">
					<Label class="text-sm font-medium text-zinc-50">Theme</Label>
					<RadioGroup.Root bind:value={selectedTheme} onValueChange={handleThemeChange} class="flex flex-row space-x-6">
						<div class="flex items-center space-x-2">
							<RadioGroup.Item value="light" id="light" />
							<Label for="light" class="flex cursor-pointer items-center text-zinc-50">
								<Sun class="mr-2 h-4 w-4" />
								Light
							</Label>
						</div>
						<div class="flex items-center space-x-2">
							<RadioGroup.Item value="dark" id="dark" />
							<Label for="dark" class="flex cursor-pointer items-center text-zinc-50">
								<Moon class="mr-2 h-4 w-4" />
								Dark
							</Label>
						</div>
						<div class="flex items-center space-x-2">
							<RadioGroup.Item value="system" id="system" />
							<Label for="system" class="flex cursor-pointer items-center text-zinc-50">
								<Monitor class="mr-2 h-4 w-4" />
								System
							</Label>
						</div>
					</RadioGroup.Root>
				</div>

				<div class="flex items-center space-x-2">
					<Checkbox id="discordRichPresence" bind:checked={discordRichPresence} />
					<Label for="discordRichPresence" class="text-zinc-50">Discord Rich Presence</Label>
				</div>
			</div>
		</div>

		<div>
			<h2 class="mb-4 text-2xl font-semibold text-zinc-50">Minecraft</h2>
			<div class="flex items-center space-x-2">
				<Checkbox id="discreteGpu" bind:checked={useDiscreteGpu} />
				<Label for="discreteGpu" class="text-zinc-50">Use discrete GPU</Label>
			</div>
			<!-- <div class="mt-4">
				<div class="inline-flex items-center">
					<Checkbox id="autoOpenLogs" bind:checked={autoOpenLogs} />
					<Label for="autoOpenLogs" class="ml-2 text-zinc-50">Auto open logs</Label>
				</div>
			</div> -->
		</div>

		<div>
			<h2 class="mb-4 text-2xl font-semibold text-zinc-50">Java</h2>
			<div class="space-y-4">
				<div class="flex space-x-4">
					<Button onclick={() => (showAutomaticJavaPopup = true)}>Set up Java automatically</Button>
					<Button onclick={() => (showManualJavaSetup = true)} variant="destructive">Set up Java manually</Button>
				</div>

				{#if showManualJavaSetup}
					<ManualJavaSetup />
				{/if}
			</div>
		</div>
		<div class="pt-4">
			<Button onclick={handleSubmit}>Save Settings</Button>
		</div>
	</div>
</div>
