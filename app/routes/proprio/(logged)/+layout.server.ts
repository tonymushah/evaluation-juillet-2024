import { PROPRIO_BACK_URL } from '$env/static/private';
import { getToken } from '$lib/proprio/token.server';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	return {
		token: getToken(cookies),
		backUrl: PROPRIO_BACK_URL
	};
};
