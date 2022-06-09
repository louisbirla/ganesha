<script lang="ts">
	import { UserStore } from '$lib/auth';
import StatementCreator from '$lib/StatementCreator.svelte';
import { Container } from '@kahi-ui/framework';
</script>

<svelte:head>
	<title>Ganesha</title>
</svelte:head>

<Container>
	{#if $UserStore}
	<p>Welcome, {$UserStore.username}!</p>
	<StatementCreator />
{:else}
	<p>You are not logged in</p>
{/if}
</Container>
