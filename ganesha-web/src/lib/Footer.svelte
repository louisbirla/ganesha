<script lang="ts">
	import { ENDPOINT_URL } from '$lib/endpoint';

	import Version from '$lib/Version.svelte';
	import { Center, Anchor, Stack } from '@kahi-ui/framework';
</script>

<footer>
	<Center>
		<Stack.Container>
			<Stack.Container orientation="horizontal" spacing="small" alignment_x="center">
				<Anchor palette="neutral" href="https://github.com/louisbirla/ganesha" target="_blank"
					>GitHub</Anchor
				>
				<Anchor palette="neutral" href={ENDPOINT_URL} target="_blank"><Version /></Anchor>
			</Stack.Container>
			<Anchor palette="neutral" href="https://birla.io" target="_blank">&copy; Louis Birla</Anchor>
		</Stack.Container>
	</Center>
</footer>
