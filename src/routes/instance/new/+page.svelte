<script lang="ts">
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { Slider } from "$lib/components/ui/slider";
	import { Button } from "$lib/components/ui/button";
	import * as Select from "$lib/components/ui/select";
	import type { Instance, Modloader, Version } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import { goto } from "$app/navigation";

	let instanceName = $state("");
	let latestReleaseVersion = $state<Version>();
	let versions = $state<Version[]>([]);
	let selectedVersion = $state<Version>();
	let selectedVersionId = $state<string>("");
	let showSnapshots = $state(false);
	let modloader = $state<Modloader>("");
	let isModloaderVersionDisabled = $state(true);
	let discordRichPresence = $state(true);
	let startMaximized = $state(false);
	let ram = $state([4096]);

	async function getVersions() {
		invoke<Version[]>("get_versions").then((data) => {
			versions = data;
			latestReleaseVersion = versions.find((v) => v.type === "release");
		});
	}

	function filterVersions(): Version[] {
		return showSnapshots ? versions : versions.filter((v) => !v.type.includes("snapshot"));
	}

	$effect(() => {
		selectedVersion = versions.find((v) => v.id === selectedVersionId);
		isModloaderVersionDisabled = modloader === "Vanilla" || modloader === undefined || modloader === "";
	});

	let versionTrigger = $derived(versions.find((v) => v.id === selectedVersion?.id)?.id ?? "Select a version");
	let modloaderTrigger = $derived(modloader ? modloader : "Select a modloader");

	let isButtonDisabled = $derived(
		!instanceName || !selectedVersionId || !modloader || (ram[0] <= 0)
	);

	async function createInstance(version: Version) {
		let instance: Instance = {
			name: instanceName,
			slug: instanceName.toLowerCase().replace(/[ /\\:]/g, "-"),
			game: {
				version: version.id,
				modloader: {
					loader: modloader,
					version: ""
				},
				url: version.url
			},
			java: {
				path: "",
				args: []
			},
			settings: {
				hasLaunched: false,
				richPresence: discordRichPresence,
				maximised: startMaximized,
				memory: parseInt(ram.join(""))
			}
		}
		await invoke("create_instance", { instance }).then(() => {
			console.log("Instance created successfully");
			goto(`/#/instance/${instance.slug}`);
		});
	}

	getVersions();
</script>

<div class="w-full overflow-hidden font-display">
	<p class="px-10 pt-10 text-3xl font-bold text-zinc-50">New Instance</p>

	<div class="flex space-x-4 px-10 py-5">
		<div>
			<Label class="mb-2 block text-zinc-50">Instance Name</Label>
			<Input type="text" bind:value={instanceName} placeholder="Minecraft {latestReleaseVersion?.id}" class="bg-background py-0" />
		</div>
		<div>
			<Label class="mb-2 block text-zinc-50">Version</Label>
			<Select.Root type="single" bind:value={selectedVersionId}>
				<Select.Trigger class="h-8 w-56">
					{versionTrigger}
				</Select.Trigger>
				<Select.Content>
					{#each filterVersions() as version (version.id)}
						<Select.Item value={version.id}>
							{version.id}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
		<div class="mt-5 inline-flex items-center space-x-2">
			<Checkbox bind:checked={showSnapshots} />
			<Label class="text-zinc-50">Snapshots</Label>
		</div>
	</div>
	<div class="flex space-x-4 px-10 py-2">
		<div>
			<Label class="mb-2 block text-zinc-50">Modloader</Label>
			<Select.Root type="single" bind:value={modloader}>
				<Select.Trigger class="h-8 w-56">
					{modloaderTrigger}
				</Select.Trigger>
				<Select.Content>
					<Select.Item value="Vanilla"></Select.Item>
					<Select.Item value="Forge">Forge</Select.Item>
					<Select.Item value="Neoforge">Neoforge</Select.Item>
					<Select.Item value="Fabric">Fabric</Select.Item>
				</Select.Content>
			</Select.Root>
		</div>
		<div>
			<Label class={`mb-2 block ${isModloaderVersionDisabled ? "cursor-not-allowed opacity-50" : "opacity-100"} text-zinc-50`}>
				Modloader Version
			</Label>
			<Select.Root type="single">
				<Select.Trigger class="h-8 w-56" disabled={isModloaderVersionDisabled}>Select a version</Select.Trigger>
				<Select.Content></Select.Content>
			</Select.Root>
		</div>
	</div>
	<div class="flex flex-col space-y-4 px-10 py-5">
		<div class="mb-2 inline-flex items-center space-x-2">
			<Checkbox bind:checked={discordRichPresence} />
			<Label class="text-zinc-50">Discord Rich Presence</Label>
		</div>
		<div class="inline-flex items-center space-x-2">
			<Checkbox bind:checked={startMaximized} />
			<Label class="text-zinc-50">Start maximised</Label>
		</div>
		<div class="inline-flex items-center space-x-5 py-2">
			<Label class="">RAM</Label>
			<Slider bind:value={ram} max={10240} step={1024} class="w-60" />
			<p>{ram} MB</p>
		</div>
		<Button onclick={() => selectedVersion && createInstance(selectedVersion)} disabled={isButtonDisabled} class="h-10 w-20">Create</Button>
	</div>
</div>
