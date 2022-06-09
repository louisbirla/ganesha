<script lang="ts">
  import { page } from '$app/stores';
import { operationStore, query } from '@urql/svelte';
import type {User} from "$lib/user"
import UserIcon from "lucide-svelte/dist/esm/icons/user.svelte"
import View from "lucide-svelte/dist/esm/icons/view.svelte"
import { Button, Card, Center, Container, Heading, Stack, Text, Tile } from '@kahi-ui/framework';
  let username = $page.params.username

  const userQuery = operationStore<{ findOneUser?: User }, {username: string}>(`
    query($username: String!) {
      findOneUser(username: $username) {
		  username
		  id
		  statements {
			  title
			  id
		  }
	  }
    }
  `, {
	  username,
  });

	query(userQuery);

	let user: User;

	userQuery.subscribe(res => {
		user = res?.data?.findOneUser
	})
</script>

<svelte:head>
	{#if user}
		 <title>{user.username} - Ganesha</title>
	{:else}
		 <title>User - Ganesha</title>
	{/if}
</svelte:head>

<Center>
	<Container>
		{#if user}
		<Stack.Container alignment_y="center" orientation="horizontal" spacing="small"><UserIcon /><Heading is="h1">{user.username}</Heading></Stack.Container>
		<Text is="small">{user.id}</Text>
		{#if user.statements.length > 0}
			  <Heading margin_y="medium" is="h2">Statements</Heading>
			  {#each user.statements as statement}
				   <Tile.Container>

    <Tile.Section>
        <Tile.Header>{statement.title}</Tile.Header>
		</Tile.Section>

    <Tile.Footer>
        <Button is="a" href="/s/{statement.id}" palette="informative">
            <View />
        </Button>
    </Tile.Footer>
</Tile.Container>
			  {/each}
		 {/if}
	{:else}
		 <Heading is="h1">User Not Found</Heading>
	{/if}
</Container>
</Center>
