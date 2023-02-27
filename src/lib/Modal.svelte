<script lang="ts">
  import { Button } from "carbon-components-svelte";
  export let open: boolean;
  export let okay: () => void;
</script>

<div class="modal">
  <div class="slot">
    <slot />
  </div>
  <div class="buttons">
    <Button on:click={() => ((open = false), okay())}>Okay</Button>
    <Button kind="danger" on:click={() => (open = false)}>Cancel</Button>
  </div>
</div>

<style>
  .modal {
    display: flex;
    flex-direction: column;
    gap: 50px;
    padding: 100px;
    width: 50vw;
    background: white;
    height: 50vh;
  }
  .slot {
    overflow-y: scroll;
  }
  .buttons {
    display: flex;
  }
</style>
