<script lang="ts">
	import { Label as LabelPrimitive } from "bits-ui";
	import { cn } from "$lib/utils";

	let { ref = $bindable(null), class: className, ...restProps }: LabelPrimitive.RootProps = $props();
</script>

<LabelPrimitive.Root
	bind:ref
	class={cn("font-display text-sm leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70", className)}
	{...restProps} />
