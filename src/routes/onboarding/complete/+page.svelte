<script lang="ts">
	import { goto } from "$app/navigation";
	import { Button } from "$lib/components/ui/button";
	import MoveRight from "lucide-svelte/icons/move-right";
	import MoveLeft from "lucide-svelte/icons/move-left";
	import { invoke } from "@tauri-apps/api/core";

	function nextPage() {
		invoke("set_onboarding_complete").then(() => {
			console.log("Onboarding complete");
			goto("/#/launcher");
		});
	}

	function previousPage() {
		goto("/#/onboarding/java");
	}
</script>

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">You're all set!</p>
	<p class="mt-4 animate-fade-in text-lg opacity-0 [animation-delay:800ms]">Glyph Launcher is ready to play Minecraft.</p>

	<Button onclick={previousPage} class="fixed bottom-4 left-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<MoveLeft class="mr-2 animate-bounce-left" />
		Back
	</Button>

	<Button onclick={nextPage} class="fixed bottom-4 right-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		Continue
		<MoveRight class="ml-2 animate-bounce-right" />
	</Button>
</div>
