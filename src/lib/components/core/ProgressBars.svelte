<script lang="ts">
	import { Progress } from "$lib/components/ui/progress";

	type Props = {
		java8Progress: number;
		java17Progress: number;
		java21Progress: number;
	};

	let { java8Progress, java17Progress, java21Progress }: Props = $props();
</script>

{#snippet Bar(progress: number, javaVersion: 8 | 17 | 21)}
	<div class="flex items-center">
		<p class="mr-4 w-16 text-zinc-50">Java {javaVersion}</p>
		<div class="flex-1">
			<Progress value={progress} />
		</div>
	</div>
{/snippet}

<div class="flex flex-col space-y-2">
	{@render Bar(java8Progress, 8)}
	{@render Bar(java17Progress, 17)}
	{@render Bar(java21Progress, 21)}
</div>
