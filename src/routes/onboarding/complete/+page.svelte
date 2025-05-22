<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import { MoveRight, MoveLeft } from "@lucide/svelte";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";
</script>

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">You're all set!</p>
	<p class="mt-4 animate-fade-in text-lg opacity-0 [animation-delay:800ms]">Glyph Launcher is ready to play Minecraft.</p>

	<Button
		onclick={() => onboardingController.navigateToPrevious()}
		class="fixed bottom-4 left-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<MoveLeft class="mr-2 animate-bounce-left" />
		Back
	</Button>

	<Button onclick={() => onboardingController.finishOnboarding()} class="fixed bottom-4 right-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		Continue
		<MoveRight class="ml-2 animate-bounce-right" />
	</Button>
</div>
