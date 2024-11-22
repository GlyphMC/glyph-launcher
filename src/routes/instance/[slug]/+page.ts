import type { PageLoad } from "./$types";

export const load = (async ({ params }) => {
	const slug = params.slug;

	return {
		slug
	};
}) satisfies PageLoad;
