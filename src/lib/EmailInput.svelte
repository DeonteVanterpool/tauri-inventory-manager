<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { TextInput } from "carbon-components-svelte";
  export let label: string;
  export let value: string;
  let valid: boolean = true;
  function input(text: CustomEvent<string>) {
    console.log("input1");
    invoke("validate_email", { inputStr: text.detail })
      .then(
        (
          v: boolean // test
        ) => ((valid = v), (value = text.detail), console.log("true"))
      )
      .catch((e) => ((valid = false), console.log(text.detail)));
    console.log("input2");
  }
</script>

<div class="input">
  <TextInput invalid={!valid} labelText={label} {value} on:input={input} />
</div>

<style lang="scss">
</style>
