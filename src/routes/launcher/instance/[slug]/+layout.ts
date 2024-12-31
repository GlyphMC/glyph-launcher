import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async ({ params }) => {
	const slug = params.slug;

	return {
		slug,
		sections: [
			{ slug: "worlds", title: "Worlds" },
			{ slug: "screenshots", title: "Screenshots" },
			{ slug: "settings", title: "Settings" },
			// TODO: Add more sections
		]
	};
}
