<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import type { MinecraftProfile } from "$lib/types";
	import { Checkbox } from "$lib/components/ui/checkbox/index";
	import { Label } from "$lib/components/ui/label/index";

	let profiles = $state<MinecraftProfile[]>([]);
	let selectedProfile = $state<MinecraftProfile>();

	async function getMinecraftProfiles() {
		await invoke<MinecraftProfile[]>("get_minecraft_profiles").then((data) => {
			profiles = data;
			selectedProfile = data[0];
		});
	}

	function selectProfile(profile: MinecraftProfile) {
		selectedProfile = profile;
	}

	$inspect(profiles, selectedProfile);

	getMinecraftProfiles();
</script>

<div class="w-full overflow-hidden font-display">
	<p class="px-10 pt-10 text-3xl font-bold text-zinc-50">Accounts</p>

	<div class="px-10 pt-5">
		{#each profiles as profile (profile.id)}
			<div class="mb-4 flex flex-col items-start">
				<div class="mb-2 flex items-center">
					<Checkbox class="mr-2" checked={selectedProfile?.id === profile.id} onclick={() => selectProfile(profile)} />
					<img
						src={`https://crafatar.com/avatars/${profile.id}`}
						alt="Profile picture of {profile.name}"
						class="mx-2 h-10 w-10 rounded border border-black" />
					<div class="flex flex-col">
						<Label class="font-bold">{profile.name}</Label>
						<Label class="mt-2 text-zinc-400">{profile.id}</Label>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
