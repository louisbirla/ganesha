<script lang="ts">
	import 'bytemd/dist/index.css'
	import "katex/dist/katex.css"
	import '@kahi-ui/framework/dist/kahi-ui.framework.min.css';
	import '@kahi-ui/framework/dist/kahi-ui.theme.default.min.css';

	import { client } from '$lib/graphql';
	import { operationStore, query, setClient } from '@urql/svelte';

	import { Button, Spacer, Stack } from '@kahi-ui/framework';
	import Logo from '$lib/Logo.svelte';
	import Footer from '$lib/Footer.svelte';
	import type { User } from '$lib/user';
	import { removeToken, UserStore } from '$lib/auth';

	setClient(client);

	const whoami = operationStore<{ whoami?: User }>(`
    query {
      whoami {
		  username
		  id
	  }
    }
  `);

	query(whoami);

	whoami.subscribe((res) => {
		UserStore.set(res.data?.whoami);
	});

	function logOut() {
		removeToken();
		UserStore.set(undefined);
	}
</script>

<Stack.Container orientation="horizontal" alignment_y="center">
	<Logo />
	<Spacer />
	{#if $UserStore}
		<div>
			<Button on:click={logOut} variation="outline" margin="nano" sizing="small">Log Out</Button>
		</div>
	{:else}
		<div>
			<Button is="a" href="/login" palette="affirmative" margin="nano" sizing="small">Log In</Button
			>
		</div>
		<div>
			<Button
				is="a"
				href="/signup"
				margin_right="small"
				variation="outline"
				palette="neutral"
				margin="nano"
				sizing="small">Sign Up</Button
			>
		</div>
	{/if}
</Stack.Container>

<slot />

<div id="footer">
	<Footer />
</div>

<style>
	#footer {
		margin-top: 10em;
	}
</style>
