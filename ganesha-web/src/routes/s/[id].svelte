<script lang="ts">
  import { page } from '$app/stores';
import { operationStore, query } from '@urql/svelte';
import { Anchor, Box, Center, Container, Heading, Stack, Text } from '@kahi-ui/framework';
import type { Statement } from '$lib/statement';
import Viewer from '$lib/Viewer.svelte';
import File from "lucide-svelte/dist/esm/icons/file-text.svelte"
  let id = $page.params.id

  const statementQuery = operationStore<{ findOneStatement?: Statement }, {id: string}>(`
    query($id: String!) {
      findOneStatement(id: $id) {
		  id
		  mdText
		  public
		  title
		  author {
			  id
			  username
		  }
	  }
    }
  `, {
	  id,
  });

	query(statementQuery);

	let statement: Statement;

	statementQuery.subscribe(res => {
		statement = res?.data?.findOneStatement
	})
</script>

<svelte:head>
	{#if statement}
		 <title>{statement.title} - Ganesha</title>
	{:else}
		 <title>Statement - Ganesha</title>
	{/if}
</svelte:head>

<Center>
	<Container>
	{#if statement}
		<Text is="small">{statement.id}</Text>
		 <Stack.Container alignment_y="center" orientation="horizontal" spacing="small"><File /><Heading is="h1">{statement.title}</Heading></Stack.Container>
		 <Text>By <Anchor title={statement.author.id} palette="affirmative" href="/u/{statement.author.username}">@{statement.author.username}</Anchor></Text>
		 <Box padding="medium">
			 <Viewer value={statement.mdText} />
		 </Box>
	{:else}
		 <Heading is="h1">Statement Not Found</Heading>
	{/if}
</Container>
</Center>
