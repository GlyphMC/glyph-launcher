<script lang="ts">
	import { afterNavigate, goto } from "$app/navigation";
	import { page } from "$app/state";
	import { onMount } from "svelte";

	onMount(async () => {
		await goto(`/#/launcher/instance/${page.params.slug}/settings`, { replaceState: true });
	});

	afterNavigate(() => {
		setTimeout(() => {
			window.dispatchEvent(new CustomEvent("force-underline-update"));
		}, 100);
	});
</script>
