<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";
	import { MoveRight } from "@lucide/svelte";
</script>

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">Welcome to Glyph Launcher</p>
	<p class="mt-2 animate-fade-in text-lg opacity-0 [animation-delay:800ms]">Let's get you set up and ready to play</p>
	<Button
		class="fixed bottom-4 right-4 animate-fade-in rounded-md px-4 py-2  opacity-0 [animation-delay:1200ms]"
		onclick={() => onboardingController.navigateToNext()}>
		<p>Continue</p>
		<MoveRight class="animate-bounce-right" />
	</Button>
</div>
