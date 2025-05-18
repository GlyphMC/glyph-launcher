<script lang="ts">
	import { onDestroy, onMount, type Snippet } from "svelte";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";
	import { page } from "$app/state";
	import { scale } from "svelte/transition";
	import { quintOut } from "svelte/easing";

	let { children }: { children: Snippet } = $props();

	onMount(async () => await onboardingController.init());
	onDestroy(() => onboardingController.cleanup());
</script>

{#key page.url.hash}
    <div
        class="h-full w-full"
        in:scale={{ start: 0.95, duration: 250, easing: quintOut }}
    >
        {@render children()}
    </div>
{/key}
