import type { Statement } from './statement';

export type User = {
	id?: string;
	username?: string;
	statements?: Statement[];
};
