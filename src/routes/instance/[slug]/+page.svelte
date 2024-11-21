<script lang="ts">
	import type { PageData } from "./$types";
	import { invoke } from "@tauri-apps/api/core";
	import type { Instance } from "$lib/types";

	let { data }: { data: PageData } = $props();
	let instance = $state<Instance>();

	invoke<Instance>("get_instance", { slug: data.slug }).then((data) => (instance = data));
</script>

<div class="overflow-hidden font-display w-full">
	<p class="px-10 pt-10 text-3xl font-bold text-zinc-50">{instance?.name}</p>
	<p class="px-10 text-zinc-50">{data.slug}</p>
</div>
