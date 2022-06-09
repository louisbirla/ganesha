<script lang="ts">
	import { setToken } from '$lib/auth';

	import { TextInput, Button, Text, Container, Stack, Heading } from '@kahi-ui/framework';

	import { mutation } from '@urql/svelte';

	let username: string | undefined;
	let password: string | undefined;
	let loading = false;
	let error: string | undefined;

	const signupMutation = mutation<
		{ signup: { token: string } },
		{ username: string; password: string }
	>({
		query: `
      mutation ($username: String!, $password: String!) {
        signup (username: $username, password: $password) {
          token
        }
      }
    `
	});

	async function signup() {
		loading = true;
		let res = await signupMutation({ username, password });
		loading = false;
		if (res.error) {
			error = res.error.message.substring(10);
		}
		if (res.data?.signup) {
			setToken(res.data.signup.token);
			location.href = '/';
		}
	}
</script>

<svelte:head>
	<title>Sign Up - Ganesha</title>
</svelte:head>

<Container width="huge">
	<Heading margin_bottom="small" is="h1">Sign Up</Heading>
	<form on:submit|preventDefault={signup}>
		<Stack.Container spacing="small">
			<TextInput placeholder="Username" bind:value={username} />
			<TextInput placeholder="Password" type="password" bind:value={password} />
			<div>
				<Button is="input" type="submit" disabled={loading} palette="informative" value="Sign Up" />
			</div>
		</Stack.Container>
		{#if error}
			<Text palette="negative">{error}</Text>
		{/if}
	</form>
</Container>
