import { route } from '$lib/ROUTES';
import { Empty } from '$lib/protos/admin';
import { DatabaseClient } from '$lib/protos/admin.client';
import { adminClient } from '$lib/server/protoclients';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	console.log('file');
	const client = new DatabaseClient(adminClient);

	const res = await client.reset(Empty);
	return redirect(300, route('/admin'));
};
