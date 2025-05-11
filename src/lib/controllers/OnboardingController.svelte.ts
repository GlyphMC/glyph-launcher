import { goto } from "$app/navigation";
import { page } from "$app/state";
import { fetchMinecraftProfiles } from "$lib/utils/AccountUtils";
import { authService } from "$lib/services/AuthService.svelte";
import type { MinecraftProfile } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import { resetMode, setMode } from "mode-watcher";

const ONBOARDING_PATHS: ReadonlyArray<string> = [
	"#/onboarding",
	"#/onboarding/theme",
	"#/onboarding/account",
	"#/onboarding/java",
	"#/onboarding/complete"
];

const LAUNCHER_PATH = "#/launcher";

export class OnboardingController {
	// Theme selection
	selectedTheme = $state<"dark" | "light" | "system">("system");

	// Account setup
	profiles = $state<MinecraftProfile[]>([]);
	selectedProfile = $state<MinecraftProfile | undefined>(undefined);

	// Java installation
	javaSetupComplete = $state(false);
	showAutomaticJavaPopUp = $state(false);
	showManualJavaEntries = $state(false);

	private _currentPath = $state("");

	constructor() {}

	async init() {
		$effect(() => {
			this._currentPath = page.url.hash;
		});

		await authService.init();
		await this.loadMinecraftProfiles();

		console.log("OnboardingController initialized");
	}

	cleanup() {
		console.log("OnboardingController cleaned up");
	}

	get showLoginPopUp() {
		return authService.showLoginPopUp;
	}

	get loginCode() {
		return authService.loginCode;
	}

	get verificationUri() {
		return authService.verificationUri;
	}

	// Theme selection
	setTheme(theme: "dark" | "light" | "system") {
		this.selectedTheme = theme;
		if (theme === "system") {
			resetMode();
		} else {
			setMode(theme);
		}
	}

	// Account setup
	async login(): Promise<void> {
		try {
			return authService.startLogin().then(async (profile) => {
				if (profile) {
					await this.loadMinecraftProfiles();
					const newProfile = this.profiles.find((p) => p.id === profile.id);
					if (newProfile) {
						this.selectedProfile = newProfile;
					} else if (this.profiles.length > 0) {
						this.selectedProfile = this.profiles[0];
					}
				}
			});
		} catch (error) {
			console.error("Login failed via OnboardingController:", error);
		}
	}

	async loadMinecraftProfiles() {
		const profilesData = await fetchMinecraftProfiles();
		this.profiles = profilesData;
		if (!this.selectedProfile && profilesData.length > 0) {
			this.selectedProfile = profilesData[0];
		} else if (profilesData.length === 0) {
			this.selectedProfile = undefined;
		}
	}

	handleCancelLoginPopUp() {
		authService.cancelLoginPopup();
	}

	selectProfile(profile: MinecraftProfile) {
		this.selectedProfile = profile;
	}

	// Java methods
	handleAutomaticJavaSetupClick() {
		this.showAutomaticJavaPopUp = true;
		this.showManualJavaEntries = false;
	}

	handleManualJavaSetupClick() {
		this.showManualJavaEntries = !this.showManualJavaEntries;
		if (this.showManualJavaEntries) {
			this.showAutomaticJavaPopUp = false;
		}
	}

	handleAutomaticJavaSetupComplete() {
		this.showAutomaticJavaPopUp = false;
		this.javaSetupComplete = true;
		this.showManualJavaEntries = false;
	}

	handleManualJavaSetupComplete() {
		this.javaSetupComplete = true;
	}

	// Navigation
	navigateToNext() {
		const currentIndex = ONBOARDING_PATHS.indexOf(this._currentPath);

		if (currentIndex < ONBOARDING_PATHS.length - 1 && currentIndex !== -1) {
			goto(ONBOARDING_PATHS[currentIndex + 1]);
		} else {
			console.warn("Cannot navigate to next path or current path is unknown:", this._currentPath);
		}
	}

	navigateToPrevious() {
		const currentIndex = ONBOARDING_PATHS.indexOf(this._currentPath);

		if (currentIndex > 0 && currentIndex !== -1) {
			goto(ONBOARDING_PATHS[currentIndex - 1]);
		} else {
			console.warn("Cannot navigate to previous path or current path is unknown:", this._currentPath);
		}
	}

	async finishOnboarding() {
		try {
			await invoke("set_onboarding_complete");
			console.log("Onboarding complete, navigating to launcher.");
			await goto(LAUNCHER_PATH);
		} catch (error) {
			console.error("Failed to set onboarding complete:", error);
		}
	}

	isNextDisabled(): boolean {
		switch (this._currentPath) {
			case ONBOARDING_PATHS[0]: // "/#/onboarding" (Welcome)
			case ONBOARDING_PATHS[1]: // "/#/onboarding/theme"
				return false;
			case ONBOARDING_PATHS[2]: // "/#/onboarding/account"
				return !this.selectedProfile || this.profiles.length === 0;
			case ONBOARDING_PATHS[3]: // "/#/onboarding/java"
				return !this.javaSetupComplete;
			case ONBOARDING_PATHS[4]: // "/#/onboarding/complete"
				return false;
			default:
				console.warn(`isNextDisabled: Unknown current path from _currentPath: "${this._currentPath}"`);
				return true;
		}
	}
}

export const onboardingController = new OnboardingController();
