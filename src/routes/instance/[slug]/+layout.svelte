<script lang="ts">
	import type { Snippet } from "svelte";
	import type { LayoutData } from "./$types";
	import type { Instance } from "$lib/types";
	import { invoke } from "@tauri-apps/api/core";

	let { data, children }: { data: LayoutData; children: Snippet } = $props();
	let instance = $state<Instance>();

	invoke<Instance>("get_instance", { slug: data.slug }).then((data) => (instance = data));
</script>

<div class="w-full overflow-hidden font-display">
	<p class="px-10 pt-10 text-3xl font-bold text-zinc-50">{instance?.name}</p>
	<p class="px-10 text-zinc-50">{data.slug}</p>
	<div>
		{#each data.sections as section}
			<a href="/instance/{data.slug}/{section.slug}">{section.title}</a>
		{/each}
	</div>
	{@render children()}
</div>
