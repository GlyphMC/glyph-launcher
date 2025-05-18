import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { LoginDetailsEvent } from "$lib/types";
import { commands, type Profile } from "$lib/bindings";

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
	async startLogin(): Promise<Profile | null> {
		if (!this.isInitialized) {
			await this.init();
		}

		const res = await commands.login();

		if (res.status === "ok") {
			this.showLoginPopUp = false;
			return res.data;
		} else {
			console.error("Login command failed:", res.error);
			this.showLoginPopUp = false;
			return null;
		}
	}

	/**
	 * Handles the cancellation of the login popup.
	 */
	async cancelLoginPopup() {
		this.showLoginPopUp = false;
		await commands.cancelLogin().then((res) => {
			if (res.status === "ok") {
				this.showLoginPopUp = false;
			} else {
				console.error("Failed to cancel login:", res.error);
			}
		});
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
