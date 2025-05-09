<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import MoveRight from "lucide-svelte/icons/move-right";
	import MoveLeft from "lucide-svelte/icons/move-left";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";
</script>

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">Choose a theme</p>

	<div class="grid animate-fade-in grid-cols-3 gap-4 p-6 opacity-0 [animation-delay:800ms]">
		<Button
			class={`flex size-28 cursor-pointer flex-col items-center justify-center p-6 transition-colors ${onboardingController.selectedTheme === "light" ? "bg-zinc-200 dark:bg-zinc-700" : ""}`}
			variant="outline"
			onclick={() => onboardingController.setTheme("light")}>
			<span class="mb-2 text-2xl">🌞</span>
			<h3 class="font-semibold">Light</h3>
		</Button>
		<Button
			class={`flex size-28 cursor-pointer flex-col items-center justify-center p-6 transition-colors ${onboardingController.selectedTheme === "dark" ? "bg-zinc-200 dark:bg-zinc-700" : ""}`}
			variant="outline"
			onclick={() => onboardingController.setTheme("dark")}>
			<span class="mb-2 text-2xl">🌙</span>
			<h3 class="font-semibold">Dark</h3>
		</Button>
		<Button
			class={`flex size-28 cursor-pointer flex-col items-center justify-center p-6 transition-colors ${onboardingController.selectedTheme === "system" ? "bg-zinc-200 dark:bg-zinc-700" : ""}`}
			variant="outline"
			onclick={() => onboardingController.setTheme("system")}>
			<span class="mb-2 text-2xl">⚙️</span>
			<h3 class="font-semibold">System</h3>
		</Button>
	</div>

	<Button
		onclick={() => onboardingController.navigateToPrevious()}
		class="fixed bottom-4 left-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<MoveLeft class="mr-2 animate-bounce-left" />
		Back
	</Button>

	<Button onclick={() => onboardingController.navigateToNext()} class="fixed bottom-4 right-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		Continue
		<MoveRight class="ml-2 animate-bounce-right" />
	</Button>
</div>
