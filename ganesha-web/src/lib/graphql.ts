import { createClient } from '@urql/svelte';
import { getToken } from './auth';
import { ENDPOINT_URL } from './endpoint';

export const client = createClient({
	url: ENDPOINT_URL,
	fetchOptions: () => {
		const token = getToken();
		return {
			headers: { authorization: token ? token : '' }
		};
	}
});
