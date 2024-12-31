import type { PageLoad } from "./$types";

export const load: PageLoad = (async ({ parent }) => {
	const parentData = await parent();
	const slug = parentData.slug;

	return {
		slug
	};
});
