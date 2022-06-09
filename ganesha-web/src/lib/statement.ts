import type { User } from './user';

export type Statement = {
	id?: string;
	mdText?: string;
	public?: string;
	title?: string;
	author?: User;
};
