<script lang="ts">
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import type { Instance, JavaConfig } from "$lib/types";
	import type { PageData } from "../$types";
	import { invoke } from "@tauri-apps/api/core";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";
	import * as Select from "$lib/components/ui/select";

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
		instance.java.version = selectedJavaVersion;

		await invoke("update_instance", { instance }).then(() => {
			console.log("Instance updated");
		});
	}

	$effect(() => {
		if (instance) {
			saveInstanceSettings();
		}
	});

	$effect(() => {
		if (instance) {
			startMaximised = instance.settings.maximised;
			windowWidth = instance.settings.windowWidth;
			windowHeight = instance.settings.windowHeight;
			selectedJavaVersion = instance.java.version;
		}
	});
</script>

<div>
	<div class="mt-4">
		<h2 class="text-lg font-semibold text-zinc-50">Minecraft</h2>
		<div class="mt-4">
			<div class="inline-flex items-center">
				<Checkbox bind:checked={startMaximised} />
				<Label class="ml-2 text-zinc-50">Start maximised</Label>
			</div>
		</div>
	</div>
	<div class="mt-4">
		<h2 class="text-lg font-semibold text-zinc-50">Java</h2>
		<Select.Root type="single" name="javaVersion" bind:value={selectedJavaVersion}>
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
