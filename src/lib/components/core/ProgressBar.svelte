<script module>
	export { ProgressBar };
</script>

<script lang="ts">
	import { Progress } from "$lib/components/ui/progress";
</script>

{#snippet ProgressBar(progress: number, label: string)}
	<div class="flex items-center">
		<p class="mr-4 w-16 text-zinc-50">{label}</p>
		<div class="flex-1">
			<Progress value={progress} />
		</div>
	</div>
{/snippet}
