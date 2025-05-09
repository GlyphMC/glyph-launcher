<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import MoveRight from "lucide-svelte/icons/move-right";
	import MoveLeft from "lucide-svelte/icons/move-left";
	import JavaDownloadPopUp from "$lib/components/core/JavaDownloadPopUp.svelte";
	import ManualJavaSetup from "$lib/components/core/ManualJavaSetup.svelte";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";
</script>

{#if onboardingController.showAutomaticJavaPopUp}
	<JavaDownloadPopUp onComplete={() => onboardingController.handleAutomaticJavaSetupComplete()} />
{/if}

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">Set up Java</p>
	<p class="mt-4 animate-fade-in text-lg opacity-0 [animation-delay:800ms]">Either choose to set up Java automatically or manually.</p>

	<div class="mt-8 flex animate-fade-in flex-row gap-4 opacity-0 [animation-delay:1000ms]">
		<Button onclick={() => onboardingController.handleAutomaticJavaSetupClick()}>Automatic Setup</Button>
		<Button variant="destructive" onclick={() => onboardingController.handleManualJavaSetupClick()}>Manual Setup</Button>
	</div>

	{#if onboardingController.showManualJavaEntries}
		<p class="mt-8 text-lg">Enter the paths to your Java installations.</p>
		<div class="mt-4">
			<ManualJavaSetup onComplete={() => onboardingController.handleManualJavaSetupComplete()} />
		</div>
	{/if}

	<Button
		onclick={() => onboardingController.navigateToPrevious()}
		class="fixed bottom-4 left-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<MoveLeft class="mr-2 animate-bounce-left" />
		Back
	</Button>

	<div class="fixed bottom-4 right-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<Button
			onclick={() => onboardingController.navigateToNext()}
			disabled={onboardingController.isNextDisabled()}
			class="transition-opacity duration-200 {onboardingController.isNextDisabled() ? 'opacity-50' : 'opacity-100'}">
			Continue
			<MoveRight class="ml-2 animate-bounce-right" />
		</Button>
	</div>
</div>
