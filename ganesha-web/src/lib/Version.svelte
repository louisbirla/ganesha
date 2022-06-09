<script lang="ts">
	import { operationStore, query } from '@urql/svelte';

	const version = operationStore<{ apiVersion: string }>(`
    query {
      apiVersion
    }
  `);

	query(version);
</script>

{#if $version.fetching}
	<span>x.x.x</span>
{:else if $version.error}
	<span>?.?.?</span>
{:else}
	<span>{$version.data.apiVersion}</span>
{/if}
