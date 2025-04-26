// src/routes/+layout.server.ts

import { buildClerkProps } from 'svelte-clerk/server';
import type { LayoutServerLoad } from './$types';

/**
 * Enables SSR support by passing the initial auth state
 * to the return object of the load function.
 */
export const load: LayoutServerLoad = ({ locals }) => {
	return {
		...buildClerkProps(locals.auth())
	};
};