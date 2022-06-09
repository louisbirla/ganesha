<script lang="ts">
  import { Viewer } from 'bytemd'
  import gfm from '@bytemd/plugin-gfm'
  import breaks from "@bytemd/plugin-breaks"
  import gemoji from "@bytemd/plugin-gemoji"
  import highlight from "@bytemd/plugin-highlight-ssr"
  import math from "@bytemd/plugin-math-ssr"
  import mermaid from "@bytemd/plugin-mermaid"

  export let value: string;

  const plugins = [
    gfm(),
    breaks(),
	gemoji(),
	highlight(),
	math(),
	mermaid(),
  ]
  </script>

<template>
  <Viewer {value} {plugins} />
</template>