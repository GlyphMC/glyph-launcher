<script lang="ts">
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import * as Select from "$lib/components/ui/select";
	import { Input } from "$lib/components/ui/input";
	import type { Instance, JavaConfig } from "$lib/types";
	import type { PageData } from "../$types";
	import { invoke } from "@tauri-apps/api/core";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";

	let { data }: { data: PageData } = $props();
	let startMaximised = $state(false);
	let windowWidth = $state(854);
	let windowHeight = $state(480);
	let instance = $state<Instance>();

	let javaConfig = $state<JavaConfig>();
	let selectedJavaVersion = $state("");
	let simpleJavaConfig = $derived(
		javaConfig
			? Object.entries(javaConfig).map(([key, path]) => ({
					version: key.replace("Path", ""),
					formattedVersion: `Java ${key.replace("Path", "").replace("java", "")}`,
					path
				}))
			: []
	);
	const triggerContent = $derived(simpleJavaConfig.find((java) => java.version === selectedJavaVersion)?.formattedVersion || "Select Java Version");

	function getInstance() {
		invoke<Instance>("get_instance", { slug: data.slug }).then((data) => {
			instance = data;

			startMaximised = data.settings.maximised;
			windowWidth = data.settings.windowWidth;
			windowHeight = data.settings.windowHeight;
			selectedJavaVersion = data.java.version;
		});
	}

	function deleteInstance() {
		console.log("Deleting instance " + data.slug);
		invoke("delete_instance", { slug: data.slug }).then(() => {
			console.log("Instance deleted");
			goto("/#/");
		});
	}

	function getJavaFromConfig() {
		invoke<JavaConfig>("get_java_from_config").then((data) => {
			javaConfig = data;
		});
	}

	onMount(() => {
		getInstance();
		getJavaFromConfig();
	});

	async function saveInstanceSettings() {
		if (!instance) return;

		instance.settings.maximised = startMaximised;
		instance.settings.windowWidth = windowWidth;
		instance.settings.windowHeight = windowHeight;
		instance.java.version = selectedJavaVersion.replace("java", "");

		await invoke("update_instance", { instance }).then(() => {
			console.log("Instance updated");
		});
	}
</script>

<div>
	<div class="mt-4">
		<h2 class="text-lg font-semibold text-zinc-50">Minecraft</h2>
		<div class="mt-4">
			<div class="inline-flex items-center">
				<Checkbox bind:checked={startMaximised} onCheckedChange={saveInstanceSettings} />
				<Label class="ml-2 text-zinc-50">Start maximised</Label>
			</div>
		</div>
		<div class="mt-4 flex space-x-4">
				<div>
					<Label class="mb-2 block text-zinc-50">Window Width</Label>
					<Input type="number" bind:value={windowWidth} onchange={saveInstanceSettings} />
				</div>
				<div>
					<Label class="mb-2 block text-zinc-50">Window Height</Label>
					<Input type="number" bind:value={windowHeight} onchange={saveInstanceSettings}/>
				</div>
			</div>
	</div>
	<div class="mt-4">
		<h2 class="text-lg font-semibold text-zinc-50">Java</h2>
		<Select.Root type="single" name="javaVersion" bind:value={selectedJavaVersion} onValueChange={saveInstanceSettings}>
			<Select.Trigger class="w-[180px]">
				{triggerContent}
			</Select.Trigger>
			<Select.Content>
				<Select.Group>
					<Select.GroupHeading>Java Versions</Select.GroupHeading>
					{#each simpleJavaConfig as java}
						<Select.Item value={java.version} label={java.formattedVersion}>
							{java.formattedVersion}
						</Select.Item>
					{/each}
				</Select.Group>
			</Select.Content>
		</Select.Root>
	</div>
	<div class="mt-4">
		<h2 class="text-lg font-semibold text-zinc-50">Advanced</h2>
		<Button class="mt-2" variant="destructive" onclick={deleteInstance}>Delete Instance</Button>
	</div>
</div>
