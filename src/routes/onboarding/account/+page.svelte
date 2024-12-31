<script lang="ts">
	import { goto } from "$app/navigation";
	import { Button } from "$lib/components/ui/button";
	import MoveRight from "lucide-svelte/icons/move-right";
	import MoveLeft from "lucide-svelte/icons/move-left";
	import { invoke } from "@tauri-apps/api/core";
	import type { LoginDetailsEvent, MinecraftProfile } from "$lib/types";
	import { listen } from "@tauri-apps/api/event";
	import { onMount } from "svelte";
	import LoginPopUp from "$lib/components/core/LoginPopUp.svelte";

	let profiles = $state<MinecraftProfile[]>([]);
	let selectedProfile = $state<MinecraftProfile>();
	let loginCode = $state("");
	let verificationUri = $state("");
	let showPopUp = $state(false);

	let isNextDisabled = $derived(!selectedProfile || profiles.length === 0);

	async function login() {
		await invoke<MinecraftProfile>("login")
			.then((data) => {
				profiles.push(data);
				selectedProfile = data;
			})
			.finally(() => (showPopUp = false));
	}

	async function getMinecraftProfiles() {
		await invoke<MinecraftProfile[]>("get_minecraft_profiles").then((data) => {
			profiles = data;
			selectedProfile = data[0];
		});
	}

	listen<LoginDetailsEvent>("login-details", (event) => {
		loginCode = event.payload.code;
		verificationUri = event.payload.uri;
		showPopUp = true;
	});

	function nextPage() {
		goto("/#/onboarding/java");
	}

	function previousPage() {
		goto("/#/onboarding/theme");
	}

	onMount(() => {
		getMinecraftProfiles();
	});
</script>

{#if showPopUp}
	<LoginPopUp {loginCode} {verificationUri} onCancel={() => (showPopUp = false)} />
{/if}

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">Sign in with Microsoft</p>
	<p class="mt-4 animate-fade-in text-lg opacity-0 [animation-delay:800ms]">Connect your Microsoft account to continue</p>

	{#if profiles.length === 0}
		<Button class="mt-8 flex animate-fade-in items-center gap-2 opacity-0 [animation-delay:1000ms]" variant="outline" onclick={login}>
			<img src="microsoft.svg" alt="Microsoft Logo" class="h-5 w-5" />
			Login with Microsoft
		</Button>
	{:else}
		<div class="mt-8 flex flex-col gap-4">
			{#each profiles as profile}
				<Button
					variant="outline"
					class="flex w-64 items-center gap-3 p-4 {selectedProfile?.id === profile.id ? 'ring-2 ring-zinc-400' : ''}"
					onclick={() => (selectedProfile = profile)}>
					<img src={`https://crafatar.com/avatars/${profile.id}?overlay=true`} alt="Avatar" class="size-6 rounded" />
					<span class="text-lg">{profile.name}</span>
				</Button>
			{/each}
		</div>
		<Button variant="outline" onclick={login} class="mt-4 w-64">Add another account</Button>
	{/if}

	<Button onclick={previousPage} class="fixed bottom-4 left-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<MoveLeft class="mr-2 animate-bounce-left" />
		Back
	</Button>

	<div class="fixed bottom-4 right-4 animate-fade-in opacity-0 [animation-delay:1200ms]">
		<Button onclick={nextPage} disabled={isNextDisabled} class="transition-opacity duration-200 {isNextDisabled ? 'opacity-50' : 'opacity-100'}">
			Continue
			<MoveRight class="ml-2 animate-bounce-right" />
		</Button>
	</div>
</div>
