<script lang="ts">
  import type { Brand } from "src/lib/types";
  import GroupBox from "../../lib/GroupBox.svelte";
  import Card from "../../lib/Card.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    Button,
    Modal,
    NumberInput,
    TextInput,
  } from "carbon-components-svelte";

  export let brands: Brand[];

  async function remove(index: number) {
    let brand = brands.splice(index, 1)[0];
    console.log(brand.id);
    await invoke("remove_brand", { id: brand.id })
      .then(() => (brands = brands))
      .catch((e) => console.log(e));
    brands = brands;
  }
  async function new_brand() {
    await invoke("new_brand")
      .then(
        (brand: Brand) =>
          (brands = [
            ...brands,
            {
              ...brand,
            },
          ])
      )
      .catch((e) => console.log(e));
  }

  let editing: boolean[] = brands.map((brand) => false);
  function edit(index: number) {
    editing[index] = true;
  }
  async function save(index: number) {
    await invoke("save_brand", { brand: brands[index] })
      .then(() => (editing[index] = false))
      .catch((e) => console.log(e));
  }
  let chooseBrand = false;
</script>

<div class="brands">
  {#each brands as brand, index}
    <Card
      editable={true}
      edit={() => edit(index)}
      save={() => save(index)}
      remove={() => remove(index)}
    >
      {#if !editing[index]}
        <GroupBox label="name" value={brand.name} />
      {:else}
        <TextInput labelText="Name" bind:value={brand.name} />
      {/if}
    </Card>
  {/each}
  <Button style="width: 500px;" kind="tertiary" on:click={new_brand}
    >+ Add</Button
  >
</div>

<style lang="scss">
  .brands {
    width: 100%;
    padding: 40px;
    display: flex;
    align-items: center;
    flex-direction: column-reverse;
    gap: 40px;
  }
</style>
