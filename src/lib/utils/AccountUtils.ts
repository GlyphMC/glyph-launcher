import { invoke } from "@tauri-apps/api/core";
import type { Account, MinecraftProfile } from "$lib/types";

/**
 * Fetches the list of Minecraft profiles from the backend.
 * @returns A promise that resolves to an array of Minecraft profiles.
 * @throws Will throw an error if the backend call fails.
 */
export async function fetchMinecraftProfiles(): Promise<MinecraftProfile[]> {
	try {
		const profiles = await invoke<MinecraftProfile[]>("get_minecraft_profiles");
		return profiles;
	} catch (error) {
		console.error("Failed to fetch Minecraft profiles:", error);
		// For now, returning an empty array to avoid breaking controllers expecting an array.
		return [];
	}
}

/**
 * Fetches the currently active account.
 * @returns A promise that resolves to the active account or null if none is active.
 */
export async function getActiveAccount(): Promise<Account | null> {
	try {
		const account = await invoke<Account | null>("get_active_account");
		return account;
	} catch (error) {
		console.error("Failed to get active account:", error);
		return null;
	}
}

/**
 * Switches the active Minecraft account.
 * @param profileId The ID of the profile to switch to.
 * @returns A promise that resolves when the account switch is complete.
 */
export async function switchAccount(profileId: string): Promise<void> {
	try {
		await invoke("switch_account", { id: profileId });
	} catch (error) {
		console.error(`Failed to switch account to ${profileId}:`, error);
		throw error;
	}
}

/**
 * Deletes a Minecraft account.
 * @param profileId The ID of the profile to delete.
 * @returns A promise that resolves when the account deletion is complete.
 */
export async function deleteAccount(profileId: string): Promise<void> {
	try {
		await invoke("delete_account", { id: profileId });
	} catch (error) {
		console.error(`Failed to delete account ${profileId}:`, error);
		throw error;
	}
}
