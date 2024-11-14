<script lang="ts">
	import { Progress } from "$lib/components/ui/progress/index";

	type Props = {
		java8Progress: number;
		java17Progress: number;
		java21Progress: number;
	};

	let { java8Progress, java17Progress, java21Progress }: Props = $props();
</script>

<div class="flex items-center">
	<p class="mr-4 w-16 text-zinc-50">Java 8</p>
	<div class="flex-1">
		<Progress value={java8Progress} />
	</div>
</div>
<div class="mt-2 flex items-center">
	<p class="mr-4 w-16 text-zinc-50">Java 17</p>
	<div class="flex-1">
		<Progress value={java17Progress} />
	</div>
</div>
<div class="mt-2 flex items-center">
	<p class="mr-4 w-16 text-zinc-50">Java 21</p>
	<div class="flex-1">
		<Progress value={java21Progress} />
	</div>
</div>
