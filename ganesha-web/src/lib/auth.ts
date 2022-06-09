import { browser } from '$app/env';
import { writable } from 'svelte/store';
import type { User } from '$lib/user';

const TOKEN_KEY = 'auth_token';

export const getToken = () => {
	if (browser) {
		return window.localStorage.getItem(TOKEN_KEY) || undefined;
	}
	return;
};

export const setToken = (value: string) => {
	if (browser) window.localStorage.setItem(TOKEN_KEY, value);
};

export const removeToken = () => {
	if (browser) window.localStorage.removeItem(TOKEN_KEY);
};

export const UserStore = writable<User>();
