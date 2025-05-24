import { goto } from "$app/navigation";
import { fetchMinecraftProfiles } from "$lib/utils/AccountUtils";
import { authService } from "$lib/services/AuthService.svelte";
import { type UnlistenFn } from "@tauri-apps/api/event";
import { commands, events, type Instance, type Profile } from "$lib/bindings";

export class SidebarController {
	instances = $state<Instance[]>([]);
	searchInput = $state("");
	profiles = $state<Profile[]>([]);
	selectedProfile = $state<Profile | undefined>(undefined);

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
		await this.setupListeners();
	}

	async fetchInstances() {
		await commands.getInstances().then((res) => {
			if (res.status === "ok") {
				this.instances = res.data;
			} else {
				console.error("Failed to fetch instances:", res.error);
				this.instances = [];
			}
		});
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
		await authService.cancelLoginPopup();
	}

	private async setupListeners() {
		this.unlistenInstanceListUpdated = await events.instanceListUpdatedEvent.listen(() => this.fetchInstances());
	}

	async logout() {
		await this.loadMinecraftProfiles();

		if (!this.selectedProfile) {
			console.warn("No profile selected to logout.");
			return;
		}

		await commands.deleteAccount(this.selectedProfile.id).then((res) => {
			if (res.status === "ok") {
				this.loadMinecraftProfiles();
			} else {
				console.error("Failed to delete account:", res.error);
			}
		});
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
