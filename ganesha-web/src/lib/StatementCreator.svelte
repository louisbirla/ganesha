<script lang="ts">
import { Button, Text, TextInput, Stack, Check, Heading, Spacer } from "@kahi-ui/framework";	
import { mutation } from "@urql/svelte";
import { goto } from '$app/navigation';
import Editor from "./Editor.svelte";
import File from "lucide-svelte/dist/esm/icons/file-text.svelte"

const createMutation = mutation<
		{ createStatement: { id: string } },
		{ text: string; title: string, is_public: boolean }
	>({
		query: `
      mutation ($text: String!, $title: String!, $is_public: Boolean!) {
        createStatement (mdText: $text, title: $title, public: $is_public) {
          id
        }
      }
    `
	});

let getData = () => ""
let title = "";
let is_public = false;
async function create() {
    let res = await createMutation({
        text: getData() || "",
        is_public,
        title,
    });
    if (res.error) {
        alert(res.error.message)
    } else {
     goto(`/s/${res.data.createStatement.id}`)
    }
}
</script>

            <Stack.Container alignment_y="center" margin_bottom="medium" orientation="horizontal" spacing="small"><File /><Heading is="h1">Create a Statement</Heading></Stack.Container>

                <TextInput style="font-weight: bold;" margin_bottom="small" bind:value={title} placeholder="Statement Title" />
				<div><Editor setGetter={(d) => {getData = d}} /></div>
    

                    
                        <Stack.Container margin_top="medium" orientation="horizontal">
                            <Spacer />
                            <Stack.Container alignment_x="right">
                                <Text style="font-weight: semibold;">Public? <Check bind:state={is_public} palette="informative" /></Text>
                                <Stack.Container margin_top="medium" orientation="horizontal">
                <Button
                    palette="inverse"
                    variation="clear"
                >
                    Cancel
                </Button>

                <Button on:click={create} palette="affirmative">
                    Create
                </Button>
                    </Stack.Container>
                    </Stack.Container>
                    </Stack.Container>