<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { TextInput } from "carbon-components-svelte";
  export let label: string;
  export let value: string;
  let valid: boolean = true;
  async function input(text: CustomEvent<string>) {
    await invoke("validate_phone_number", { inputStr: text.detail })
      .then((v: boolean) => ((valid = v), (value = text.detail)))
      .catch((e) => (valid = false));
  }
</script>

<div class="input">
  <TextInput invalid={!valid} labelText={label} {value} on:input={input} />
</div>

<style lang="scss">
</style>
