<script lang="ts">
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { Textarea } from "$lib/components/ui/textarea";
	import * as Select from "$lib/components/ui/select";
	import type { Instance, JavaConfig } from "$lib/types";
	import type { PageData } from "../$types";
	import { invoke } from "@tauri-apps/api/core";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";

	let { data }: { data: PageData } = $props();
	let startMaximized = $state(false);
	let windowWidth = $state(854);
	let windowHeight = $state(480);
	let windowDimensionsDisabled = $derived(startMaximized);
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
	const triggerContent = $derived(selectedJavaVersion ? `Java ${selectedJavaVersion.replace("java", "")}` : "Select Java Version");
	let javaArguments = $state("");

	async function getInstance() {
		await invoke<Instance>("get_instance", { slug: data.slug }).then((data) => {
			instance = data;

			startMaximized = data.settings.maximized;
			windowWidth = data.settings.windowWidth;
			windowHeight = data.settings.windowHeight;
			selectedJavaVersion = `java${data.java.version}`;
		});
	}

	async function deleteInstance() {
		console.log("Deleting instance " + data.slug);
		await invoke("delete_instance", { slug: data.slug }).then(() => {
			console.log("Instance deleted");
			goto("/#/launcher/");
		});
	}

	async function getJavaFromConfig() {
		await invoke<JavaConfig>("get_java_from_config").then((data) => {
			javaConfig = data;
		});
	}

	onMount(async () => {
		await getInstance();
		await getJavaFromConfig();
	});

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		if (!instance) return;

		instance.settings.maximized = startMaximized;
		instance.settings.windowWidth = windowWidth;
		instance.settings.windowHeight = windowHeight;
		instance.java.version = parseInt(selectedJavaVersion.replace("java", ""), 10);
		instance.java.path = simpleJavaConfig.find((java) => java.version === selectedJavaVersion)?.path || "";
		instance.java.args = javaArguments.split(" ").filter((arg) => arg.trim() !== "");

		await invoke("update_instance", { instance }).then(() => {
			console.log("Instance updated");
		});
	}
</script>

<form onsubmit={handleSubmit}>
	<h2 class="text-lg font-semibold text-zinc-50">Minecraft</h2>
	<div class="mt-4">
		<div class="inline-flex items-center">
			<Checkbox bind:checked={startMaximized} />
			<Label class="ml-2 text-zinc-50">Start maximised</Label>
		</div>
	</div>
	<div class="mt-4 flex space-x-4">
		<div>
			<Label class={`mb-2 block text-zinc-50 ${windowDimensionsDisabled ? "opacity-50" : ""}`}>Window Width</Label>
			<Input type="number" bind:value={windowWidth} disabled={windowDimensionsDisabled} />
		</div>
		<div>
			<Label class={`mb-2 block text-zinc-50 ${windowDimensionsDisabled ? "opacity-50" : ""}`}>Window Height</Label>
			<Input type="number" bind:value={windowHeight} disabled={windowDimensionsDisabled} />
		</div>
	</div>
	<div class="mt-4 flex flex-col">
		<div>
			<h2 class="text-lg font-semibold text-zinc-50">Java</h2>
			<Select.Root type="single" name="javaVersion" bind:value={selectedJavaVersion}>
				<Select.Trigger class="mt-2 w-[180px]">
					{triggerContent}
				</Select.Trigger>
				<Select.Content>
					<Select.Group>
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
			<Label class="mb-2 block text-zinc-50">Java Arguments</Label>
			<Textarea class="resize-none" placeholder="Enter Java Arguments here" bind:value={javaArguments} />
		</div>
	</div>
	<div class="mt-8 flex items-center space-x-4">
		<Button type="submit">Save Settings</Button>
	</div>
</form>

<div class="mt-8 border-t border-zinc-700 pt-4">
	<h2 class="text-lg font-semibold text-zinc-50">Advanced</h2>
	<Button class="mt-2" variant="destructive" onclick={deleteInstance}>Delete Instance</Button>
</div>
