<script lang="ts">
	import { onDestroy, onMount, type Snippet } from "svelte";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";

	let { children }: { children: Snippet } = $props();

	onMount(async () => await onboardingController.init());
	onDestroy(() => onboardingController.cleanup());
</script>

{@render children()}
