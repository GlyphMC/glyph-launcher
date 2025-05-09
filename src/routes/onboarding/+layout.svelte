<script lang="ts">
	import { onDestroy, onMount, type Snippet } from "svelte";
	import type { LayoutData } from "./$types";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";

	let { data, children }: { data: LayoutData; children: Snippet } = $props();

	onMount(async () => {
		await onboardingController.init();
	});

	onDestroy(() => {
		onboardingController.cleanup();
	});
</script>

{@render children()}
