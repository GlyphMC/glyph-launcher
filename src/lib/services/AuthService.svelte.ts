import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { LoginDetailsEvent, MinecraftProfile } from "$lib/types";

class AuthService {
	loginCode = $state("");
	verificationUri = $state("");
	showLoginPopUp = $state(false);

	private unlistenLoginDetails: UnlistenFn | undefined;
	private isInitialized = false;

	constructor() {}

	async init() {
		if (this.isInitialized) return;
		this.isInitialized = true;

		this.unlistenLoginDetails = await listen<LoginDetailsEvent>("login-details", (event) => {
			this.loginCode = event.payload.code;
			this.verificationUri = event.payload.uri;
			this.showLoginPopUp = true;
		});
		console.log("AuthService initialized and listener set up.");
	}

	/**
	 * Initiates the login process.
	 * The "login" Tauri command is expected to first emit "login-details"
	 * (which shows the popup via the listener in init) and then resolve
	 * with the MinecraftProfile upon successful authentication.
	 * @returns A Promise that resolves with the MinecraftProfile if login is successful, otherwise null.
	 */
	async startLogin(): Promise<MinecraftProfile | null> {
		try {
			// Ensure listener is active
			if (!this.isInitialized) {
				await this.init();
			}

			const profile = await invoke<MinecraftProfile>("login");
			this.showLoginPopUp = false;

			return profile;
		} catch (error) {
			console.error("Login process failed:", error);
			return null;
		}
	}

	/**
	 * Handles the cancellation of the login popup.
	 */
	async cancelLoginPopup() {
		this.showLoginPopUp = false;
		try {
			await invoke("cancel_login");
			console.log("Login flow cancelled via AuthService.");
		} catch (error) {
			console.error("Failed to cancel login via Tauri:", error);
		}
	}

	cleanup() {
		this.unlistenLoginDetails?.();
		this.isInitialized = false;
		this.showLoginPopUp = false;
		this.loginCode = "";
		this.verificationUri = "";
		console.log("AuthService cleaned up.");
	}
}

export const authService = new AuthService();
