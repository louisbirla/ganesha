<script lang="ts">
  import { Editor } from 'bytemd'
  import gfm from '@bytemd/plugin-gfm'
  import breaks from "@bytemd/plugin-breaks"
  import gemoji from "@bytemd/plugin-gemoji"
  import highlight from "@bytemd/plugin-highlight-ssr"
  import math from "@bytemd/plugin-math-ssr"
  import mermaid from "@bytemd/plugin-mermaid"

  export let setGetter = (fn: () => string) => {};

  let value: string;

  const plugins = [
    gfm(),
    breaks(),
	gemoji(),
	highlight(),
	math(),
	mermaid(),
  ]

  function handleChange(e) {
    value = e.detail.value
  }

  setGetter(() => value)
</script>

<template>
  <Editor {value} {plugins} on:change={handleChange} />
</template>