<script lang="ts">
	import { Button } from "$lib/components/ui/button";
	import MoveRight from "lucide-svelte/icons/move-right";
	import MoveLeft from "lucide-svelte/icons/move-left";
	import LoginPopUp from "$lib/components/core/LoginPopUp.svelte";
	import { onboardingController } from "$lib/controllers/OnboardingController.svelte";
	import { useAvatar } from "$lib/utils/AvatarUtils";
</script>

{#if onboardingController.showLoginPopUp}
	<LoginPopUp
		loginCode={onboardingController.loginCode}
		verificationUri={onboardingController.verificationUri}
		onCancel={() => onboardingController.handleCancelLoginPopUp()} />
{/if}

<div class="group flex min-h-screen select-none flex-col items-center justify-center font-display">
	<p class="animate-fade-in text-5xl font-bold opacity-0">Sign in with Microsoft</p>
	<p class="mt-4 animate-fade-in text-lg opacity-0 [animation-delay:800ms]">Connect your Microsoft account to continue</p>

	{#if onboardingController.profiles.length === 0}
		<Button
			class="mt-8 flex animate-fade-in items-center gap-2 opacity-0 [animation-delay:1000ms]"
			variant="outline"
			onclick={async () => await onboardingController.login()}>
			<img src="/microsoft.svg" alt="Microsoft Logo" class="h-5 w-5" />
			Login with Microsoft
		</Button>
	{:else}
		<div class="mt-8 flex flex-col gap-4">
			{#each onboardingController.profiles as profile (profile.id)}
				<Button
					variant="outline"
					class="flex w-64 items-center gap-3 p-4 {onboardingController.selectedProfile?.id === profile.id ? 'ring-2 ring-zinc-400' : ''}"
					onclick={() => onboardingController.selectProfile(profile)}>
					{#await useAvatar(profile.id)}
						<div class="size-6 animate-pulse rounded bg-zinc-700"></div>
					{:then avatarSrc}
						{#if avatarSrc}
							<img src={avatarSrc} alt="Avatar for {profile.name}" class="size-6 rounded" />
						{:else}
							<div class="size-6 rounded bg-zinc-700"></div>
						{/if}
					{:catch _}
						<div class="size-6 rounded bg-zinc-700"></div>
					{/await}
				</Button>
			{/each}
		</div>
		<Button variant="outline" onclick={async () => await onboardingController.login()} class="mt-4 w-64">Add another account</Button>
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
