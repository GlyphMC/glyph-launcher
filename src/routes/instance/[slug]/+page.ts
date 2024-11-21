import { invoke } from "@tauri-apps/api/core";
import type { EntryGenerator, PageLoad } from "./$types";
import type { Instance } from "$lib/types";
import { browser } from "$app/environment";

export const load = (async ({ params }) => {
	const slug = params.slug;

	return {
		slug
	};
}) satisfies PageLoad;

export const entries: EntryGenerator = async () => {
	if (browser) {
		const data = await invoke<Instance[]>("get_instances");
		return data.map((instance) => ({
			slug: instance.slug
		}));
	} else {
		return [];
	}
};
