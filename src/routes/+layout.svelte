<!--
The <ClerkProvider> component wraps the entire application and provides 
context for user authentication. It ensures that all child components 
have access to Clerk's authentication state, session, and user data.

This is required for using Clerk's prebuilt components, runes, and helpers 
throughout the app. All components that rely on Clerk must be nested 
within this provider.

For more details, refer to the Svelte Clerk documentation:
https://svelte-clerk.netlify.app/ssr/quickstart.html
-->

<script lang="ts">
  import "./global.css";
	import type { Snippet } from 'svelte';
	import { ClerkProvider } from 'svelte-clerk';

	const { children }: { children: Snippet } = $props();
</script>

<ClerkProvider>
	{@render children()}
</ClerkProvider>
