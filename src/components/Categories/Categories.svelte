<script lang="ts">
  import type { Category } from "src/lib/types";
  import GroupBox from "../../lib/GroupBox.svelte";
  import Card from "../../lib/Card.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    Button,
    Modal,
    NumberInput,
    TextInput,
  } from "carbon-components-svelte";

  export let categories: Category[];

  async function remove(index: number) {
    let category = categories.splice(index, 1)[0];
    console.log(category.id);
    await invoke("remove_category", { id: category.id })
      .then(() => (categories = categories))
      .catch((e) => console.log(e));
    categories = categories;
  }
  async function new_category() {
    await invoke("new_category")
      .then(
        (category: Category) =>
          (categories = [
            ...categories,
            {
              ...category,
            },
          ])
      )
      .catch((e) => console.log(e));
  }

  let editing: boolean[] = categories.map((category) => false);
  function edit(index: number) {
    editing[index] = true;
  }
  async function save(index: number) {
    await invoke("save_category", { category: categories[index] })
      .then(() => (editing[index] = false))
      .catch((e) => console.log(e));
  }
  let chooseCategory = false;
</script>

<div class="categories">
  {#each categories as category, index}
    <Card
      editable={true}
      edit={() => edit(index)}
      save={() => save(index)}
      remove={() => remove(index)}
    >
      {#if !editing[index]}
        <GroupBox label="name" value={category.name} />
      {:else}
        <TextInput labelText="Name" bind:value={category.name} />
      {/if}
    </Card>
  {/each}
  <Button style="width: 500px;" kind="tertiary" on:click={new_category}
    >+ Add</Button
  >
</div>

<style lang="scss">
  .categories {
    width: 100%;
    padding: 40px;
    display: flex;
    align-items: center;
    flex-direction: column-reverse;
    gap: 40px;
  }
</style>
