import { route } from '$lib/ROUTES';
import generateTokenHandler from '$lib/generateTokenHandler';
import { CLIENT_TOKEN_KEY as TOKEN_KEY } from './token.key';

const { getTokenOrRedirect, setToken } = generateTokenHandler(TOKEN_KEY);

export { setToken };

export function getToken() {
	return getTokenOrRedirect(route('/client/login'));
}
