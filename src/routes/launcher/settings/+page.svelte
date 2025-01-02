<script lang="ts">
	import { platform } from "@tauri-apps/plugin-os";
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import { resetMode, setMode } from "mode-watcher";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import JavaDownloadPopUp from "$lib/components/core/JavaDownloadPopUp.svelte";
	import ManualJavaSetup from "$lib/components/core/ManualJavaSetup.svelte";

	let isLinux = platform() === "linux";
	let useDiscreteGPU = $state(false);

	let showAutomaticJavaPopup = $state(false);

	let showManualJavaSetup = $state(false);
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
			<ManualJavaSetup />
		{/if}
	</div>
</div>
