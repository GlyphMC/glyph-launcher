<script lang="ts">
	import { goto } from "$app/navigation";
	import { Button } from "$lib/components/ui/button";
	import MoveRight from "lucide-svelte/icons/move-right";
	import MoveLeft from "lucide-svelte/icons/move-left";
	import { resetMode, setMode } from "mode-watcher";

	let selectedTheme = $state("system");

	function setTheme(theme: "dark" | "light" | "system") {
		selectedTheme = theme;
		if (theme === "system") {
			resetMode();
		} else {
			setMode(theme);
		}
	}

	function nextPage() {
		goto("/#/onboarding/account");
	}

	function previousPage() {
		goto("/#/onboarding/");
	}
</script>

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">Choose a theme</p>

	<div class="animate-fade-in grid grid-cols-3 gap-4 p-6 opacity-0 [animation-delay:800ms]">
		<Button
			class={`flex size-28 cursor-pointer flex-col items-center justify-center p-6 transition-colors ${selectedTheme === "light" ? "bg-zinc-200 dark:bg-zinc-700" : ""}`}
			variant="outline"
			onclick={() => setTheme("light")}>
			<span class="mb-2 text-2xl">🌞</span>
			<h3 class="font-semibold">Light</h3>
		</Button>
		<Button
			class={`flex size-28 cursor-pointer flex-col items-center justify-center p-6 transition-colors ${selectedTheme === "dark" ? "bg-zinc-200 dark:bg-zinc-700" : ""}`}
			variant="outline"
			onclick={() => setTheme("dark")}>
			<span class="mb-2 text-2xl">🌙</span>
			<h3 class="font-semibold">Dark</h3>
		</Button>
		<Button
			class={`flex size-28 cursor-pointer flex-col items-center justify-center p-6 transition-colors ${selectedTheme === "system" ? "bg-zinc-200 dark:bg-zinc-700" : ""}`}
			variant="outline"
			onclick={() => setTheme("system")}>
			<span class="mb-2 text-2xl">⚙️</span>
			<h3 class="font-semibold">System</h3>
		</Button>
	</div>

	<Button onclick={previousPage} class="animate-fade-in fixed bottom-4 left-4 opacity-0 [animation-delay:1200ms]">
		<MoveLeft class="animate-bounce-left mr-2" />
		Back
	</Button>

	<Button onclick={nextPage} class="animate-fade-in fixed bottom-4 right-4 opacity-0 [animation-delay:1200ms]">
		Continue
		<MoveRight class="animate-bounce-right ml-2" />
	</Button>
</div>
