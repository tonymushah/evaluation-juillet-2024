import type { Cookies } from '@sveltejs/kit';

// place files you want to import through the `$lib` alias in this folder.
export function generateTokenHandler(KEY: string, path?: string) {
	return {
		hasToken(cookies: Cookies): boolean {
			return cookies.get(KEY) != undefined;
		},
		getToken(cookies: Cookies): string {
			const token = cookies.get(KEY);
			if (token == undefined) {
				throw new Error('Token not found');
			} else {
				return token;
			}
		},
		setToken(cookies: Cookies, value: string) {
			cookies.set(KEY, value, {
				path: path ?? '/'
			});
		},
		deleteToken(cookies: Cookies) {
			cookies.delete(KEY, {
				path: path ?? '/'
			});
		}
	};
}
