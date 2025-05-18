import { commands, type Account, type Profile } from "$lib/bindings";

/**
 * Fetches the list of Minecraft profiles from the backend.
 * @returns A promise that resolves to an array of Minecraft profiles.
 */
export async function fetchMinecraftProfiles(): Promise<Profile[]> {
	const res = await commands.getMinecraftProfiles();
	if (res.status === "ok") {
		return res.data;
	} else {
		console.error("Failed to fetch Minecraft profiles:", res.error);
		return [];
	}
}

/**
 * Fetches the currently active account.
 * @returns A promise that resolves to the active account or null if none is active.
 */
export async function getActiveAccount(): Promise<Account | null> {
	const res = await commands.getActiveAccount();
	if (res.status === "ok") {
		return res.data;
	} else {
		console.error("Failed to fetch active account:", res.error);
		return null;
	}
}

/**
 * Switches the active Minecraft account.
 * @param profileId The ID of the profile to switch to.
 * @returns A promise that resolves when the account switch is complete.
 */
export async function switchAccount(profileId: string): Promise<void> {
	await commands.switchAccount(profileId).then((res) => {
		if (res.status === "ok") {
			console.log(`Switched to account ${profileId}`);
		} else {
			console.error(`Failed to switch account to ${profileId}:`, res.error);
		}
	});
}

/**
 * Deletes a Minecraft account.
 * @param profileId The ID of the profile to delete.
 * @returns A promise that resolves when the account deletion is complete.
 */
export async function deleteAccount(profileId: string): Promise<void> {
	await commands.deleteAccount(profileId).then((res) => {
		if (res.status === "ok") {
			console.log(`Deleted account ${profileId}`);
		} else {
			console.error(`Failed to delete account ${profileId}:`, res.error);
		}
	});
}
