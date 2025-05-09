<script lang="ts">
	import type { MinecraftProfile } from "$lib/types";
	import { Button } from "$lib/components/ui/button";
	import { Checkbox } from "$lib/components/ui/checkbox";
	import { Label } from "$lib/components/ui/label";
	import LoginPopUp from "$lib/components/core/LoginPopUp.svelte";
	import { authService } from "$lib/services/AuthService.svelte";
	import { deleteAccount, fetchMinecraftProfiles, getActiveAccount, switchAccount } from "$lib/utils/AccountUtils";

	let profiles = $state<MinecraftProfile[]>([]);
	let activeProfileId = $state<string | undefined>(undefined);

	async function loadAccountData() {
		try {
			profiles = await fetchMinecraftProfiles();
			const activeAccount = await getActiveAccount();
			activeProfileId = activeAccount?.profile.id;
		} catch (error) {
			console.error("Failed to load account data:", error);
			profiles = [];
			activeProfileId = undefined;
		}
	}

	async function handleSwitchAccount(profileId: string) {
		if (activeProfileId === profileId) return;

		try {
			await switchAccount(profileId);
			await loadAccountData();
		} catch (error) {
			console.error("Failed to switch account:", error);
		}
	}

	async function handleAddAccount() {
		try {
			const newProfile = await authService.startLogin();
			if (newProfile) {
				await loadAccountData();
			}
		} catch (error) {
			console.error("Failed to add account on page:", error);
		}
	}

	async function handleDeleteAccount(profileId: string) {
		if (profiles.length === 1 && profileId === activeProfileId) {
			console.warn("Cannot delete the only active account directly from this page. You can log out via settings or add another account first.");
			alert("Cannot delete the only account. Add another account first.");
			return;
		}
		try {
			await deleteAccount(profileId);
			await loadAccountData();
		} catch (error) {
			console.error("Failed to delete account:", error);
		}
	}

	$effect(() => {
		authService.init();
		loadAccountData();
	});
</script>

{#if authService.showLoginPopUp}
	<LoginPopUp loginCode={authService.loginCode} verificationUri={authService.verificationUri} onCancel={() => authService.cancelLoginPopup()} />
{/if}

<div class="w-full overflow-hidden px-10 font-display">
	<p class="mb-2 pt-10 text-3xl font-bold text-zinc-50">Accounts</p>

	<div class="space-y-8">
		<Button class="" onclick={handleAddAccount}>Add Account</Button>
		{#each profiles as profile (profile.id)}
			<div class="mb-4 flex flex-col items-start">
				<div class="mb-2 flex items-center">
					<Checkbox
						class="mr-4 h-5 w-5 border-zinc-600 data-[state=checked]:bg-blue-500 data-[state=checked]:text-white"
						checked={activeProfileId === profile.id}
						onclick={() => handleSwitchAccount(profile.id)}
						id={`profile-checkbox-${profile.id}`}
						aria-label={`Set ${profile.name} as active account`} />
					<img
						src={`https://crafatar.com/avatars/${profile.id}?size=40&overlay`}
						alt="Profile avatar for {profile.name}"
						class="h-10 w-10 rounded" />
					<div class="ml-4 flex flex-col">
						<Label for={`profile-checkbox-${profile.id}`} class="cursor-pointer font-semibold text-zinc-50">{profile.name}</Label>
						<Label class="mt-1 text-xs text-zinc-400">{profile.id}</Label>
					</div>
				</div>

				<Button
					variant="destructive"
					class="mt-2"
					onclick={() => handleDeleteAccount(profile.id)}
					disabled={profiles.length === 1 && activeProfileId === profile.id}
					aria-label={`Delete ${profile.name} account`}>
					Delete Account
				</Button>
			</div>
		{/each}
	</div>
</div>
