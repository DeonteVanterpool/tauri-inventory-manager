<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { TextInput } from "carbon-components-svelte";
  export let label: string;
  export let value: number;
  let valid: boolean = true;
  async function input(text: CustomEvent<string>) {
    await invoke("calc", { inputStr: text.detail })
      .then((v: number) => ((valid = true), (value = v)))
      .catch((e) => (valid = false));
  }
  let text: string = value.toString();
</script>

<div class="input">
  <TextInput invalid={!valid} labelText={label} value={text} on:input={input} />
</div>

<style lang="scss">
</style>
