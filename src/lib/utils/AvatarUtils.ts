import { commands } from "$lib/bindings";

const AVATAR_CACHE_PREFIX = "avatar_cache";
const CACHE_DURATION = 1000 * 60 * 60; // 1 hour

interface CachedAvatar {
	dataUrl: string;
	timestamp: number;
}

export async function useAvatar(uuid: string | undefined | null): Promise<string> {
	if (!uuid) return "";

	const cacheKey = `${AVATAR_CACHE_PREFIX}${uuid}`;

	try {
		const cachedItemString = localStorage.getItem(cacheKey);
		if (cachedItemString) {
			try {
				const cachedItem: CachedAvatar = JSON.parse(cachedItemString);
				const now = Date.now();

				if (now - cachedItem.timestamp < CACHE_DURATION) {
					return cachedItem.dataUrl;
				} else {
					localStorage.removeItem(cacheKey);
				}
			} catch (parseError) {
				console.error("Error parsing cached avatar item from localStorage:", parseError);
				localStorage.removeItem(cacheKey);
			}
		}
	} catch (e) {
		console.error("Error reading avatar data URL from localStorage:", e);
	}

	try {
		const res = await commands.getAvatar(uuid);
		if (res.status === "ok" && res.data) {
			const newDataUrl = res.data;
			const newItem: CachedAvatar = {
				dataUrl: newDataUrl,
				timestamp: Date.now()
			};

			try {
				localStorage.setItem(cacheKey, JSON.stringify(newItem));
			} catch (e) {
				console.error("Error writing avatar data URL to localStorage:", e);
			}
			return newDataUrl;
		}

		return "";
	} catch (error) {
		console.error(`Error fetching avatar data URL for ${uuid} via Tauri command:`, error);
		return "";
	}
}
