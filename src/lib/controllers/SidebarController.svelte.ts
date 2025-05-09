import { goto } from "$app/navigation";
import { fetchMinecraftProfiles } from "$lib/utils/AccountUtils";
import { authService } from "$lib/services/AuthService.svelte";
import type { Instance, MinecraftProfile } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export class SidebarController {
	instances = $state<Instance[]>([]);
	searchInput = $state("");
	profiles = $state<MinecraftProfile[]>([]);
	selectedProfile = $state<MinecraftProfile | undefined>(undefined);

	filteredInstances = $derived(() => {
		if (this.searchInput.trim() === "") {
			return this.instances;
		}
		return this.instances.filter((instance) => instance.name.toLowerCase().includes(this.searchInput.toLowerCase()));
	});

	private unlistenInstanceListUpdated: UnlistenFn | undefined;

	constructor() {}

	async init() {
		await authService.init();
		await this.fetchInstances();
		await this.loadMinecraftProfiles();
		this.setupListeners();
	}

	async fetchInstances() {
		try {
			const data = await invoke<Instance[]>("get_instances");
			this.instances = data;
		} catch (error) {
			console.error("Failed to fetch instances:", error);
			this.instances = [];
		}
	}

	async loadMinecraftProfiles() {
		const profilesData = await fetchMinecraftProfiles();
		this.profiles = profilesData;
		this.selectedProfile = profilesData.length > 0 ? profilesData[0] : undefined;
	}

	async login() {
		try {
			const profile = await authService.startLogin();
			if (profile) {
				await this.loadMinecraftProfiles();
				const newProfile = this.profiles.find((p) => p.id === profile.id);
				if (newProfile) {
					this.selectedProfile = newProfile;
				} else if (this.profiles.length > 0) {
					this.selectedProfile = this.profiles[0];
				}
			}
		} catch (error) {
			console.error("Login failed via SidebarController:", error);
		}
	}

	async handleCancelLoginPopUp() {
		authService.cancelLoginPopup();
	}

	private async setupListeners() {
		this.unlistenInstanceListUpdated = await listen("instance-list-updated", () => {
			this.fetchInstances();
		});
	}

	async logout() {
		console.log("logout");
		if (!this.selectedProfile) {
			console.warn("No profile selected to logout.");
			return;
		}

		try {
			await invoke("delete_account", { id: this.selectedProfile.id }).then(() => {
				console.log("Account deleted successfully");
			});
			await this.loadMinecraftProfiles();
		} catch (error) {
			console.error("Logout failed:", error);
		}
	}

	handleAddInstanceClick() {
		goto("/#/launcher/instance/new");
	}

	navigateToAccounts() {
		goto("/#/launcher/accounts");
	}

	navigateToSettings() {
		goto("/#/launcher/settings");
	}

	cleanup() {
		this.unlistenInstanceListUpdated?.();
	}
}
