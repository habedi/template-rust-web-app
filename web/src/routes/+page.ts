import { listItems } from '$lib/services/api';
import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
	return {
		title: 'Items',
		items: await listItems(fetch),
	};
}) satisfies PageLoad;
