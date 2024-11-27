<script lang="ts">
	import { onMount, type Snippet } from "svelte";
	import type { LayoutData } from "./$types";
	import type { Instance } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";
	import Clock from "lucide-svelte/icons/clock";
	import { Button } from "$lib/components/ui/button";

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
	let instance = $state<Instance>();
	let timePlayed = $state(0);

	async function fetchTimePlayed() {}

	async function launchInstance() {}

	function getInstance() {
		invoke<Instance>("get_instance", { slug: data.slug }).then((data) => {
			instance = data;
		});
	}

	onMount(() => {
		fetchTimePlayed();
		getInstance();
	});
</script>

<div class="w-full overflow-hidden font-display">
	<p class="mb-2 px-10 pt-10 text-3xl font-bold text-zinc-50">{instance?.name}</p>

	<div class="flex items-center px-10 text-sm text-zinc-300">
		<Clock class="mr-2" />
		<p>{timePlayed} played</p>
	</div>

	<Button class="ml-10 mt-8 w-24 px-10" onclick={launchInstance}>Launch</Button>

	<div class="mt-5 flex space-x-10 px-10 text-xl font-bold">
		{#each data.sections as section}
			<a href="/instance/{data.slug}/{section.slug}">{section.title}</a>
		{/each}
	</div>

	<div class="px-10">
		{@render children()}
	</div>
</div>
