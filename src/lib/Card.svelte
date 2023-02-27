<script lang="ts">
  import { Button } from "carbon-components-svelte";

  export let editable: boolean = false;
  export let remove: () => void = () => {};
  export let editing: boolean = false;

  export let edit: () => void = () => {};
  export let save: () => void = () => {};
</script>

<div class="card">
  {#if editable}
    <div class="edit-remove">
      {#if editing}
        <Button
          on:click={() => {
            editing = !editing;
            save();
          }}>Save</Button
        >
      {:else}
        <Button
          on:click={() => {
            editing = !editing;
            edit();
          }}>Edit</Button
        >
      {/if}
      <Button kind="secondary" on:click={remove}>Remove</Button>
    </div>
  {/if}
  <div class="grid">
    <slot />
  </div>
</div>

<style lang="scss">
  .edit-remove {
    display: flex;
    justify-content: right;
  }
  .card {
    width: calc(100% - 50px);
    border: 3px solid rgb(224, 224, 224);
    padding: 30px;
    border-radius: 25px;
    background-color: white;
    box-shadow: 2px 2px 5px rgb(224, 224, 224);
    align-self: center;
  }
  .grid {
    display: grid;
    gap: 20px;
    padding: 30px;
    padding-bottom: 0px;
    @media (max-width: 800px) {
      grid-template-columns: repeat(1, 1fr);
    }
    @media (min-width: 800px) and (max-width: 1200px) {
      grid-template-columns: repeat(2, 1fr);
    }
    @media (min-width: 1200px) {
      grid-template-columns: repeat(3, 1fr);
    }
  }
</style>
