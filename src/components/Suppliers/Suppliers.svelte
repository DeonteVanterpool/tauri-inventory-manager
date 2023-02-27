<script lang="ts">
  import type { Supplier } from "src/lib/types";
  import GroupBox from "../../lib/GroupBox.svelte";
  import Card from "../../lib/Card.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, Modal, TextInput } from "carbon-components-svelte";
  import NumberInput from "../../lib/NumberInput.svelte";
  import PhoneNumberInput from "../../lib/PhoneNumberInput.svelte";
  import EmailInput from "../../lib/EmailInput.svelte";

  export let suppliers: Supplier[];

  async function remove(index: number) {
    let supplier = suppliers.splice(index, 1)[0];
    console.log(supplier.id);
    await invoke("remove_supplier", { id: supplier.id })
      .then(() => (suppliers = suppliers))
      .catch((e) => console.log(e));
    suppliers = suppliers;
  }
  async function new_supplier() {
    await invoke("new_supplier")
      .then(
        (supplier: Supplier) =>
          (suppliers = [
            ...suppliers,
            {
              ...supplier,
            },
          ])
      )
      .catch((e) => console.log(e));
  }

  let editing: boolean[] = suppliers.map((supplier) => false);
  function edit(index: number) {
    editing[index] = true;
  }
  async function save(index: number) {
    await invoke("save_supplier", { supplier: suppliers[index] }) // arst
      .then(() => (editing[index] = false))
      .catch((e) => console.log(suppliers[index], e));
  }
  let chooseSupplier = false;
</script>

<div class="suppliers">
  {#each suppliers as supplier, index}
    <Card
      editable={true}
      edit={() => edit(index)}
      save={() => save(index)}
      remove={() => remove(index)}
    >
      {#if !editing[index]}
        <GroupBox label="name" value={supplier.name} />
        <GroupBox label="phone number" value={supplier.phoneNumber} />
        <GroupBox label="email" value={supplier.email} />
      {:else}
        <TextInput labelText="Name" bind:value={supplier.name} />
        <PhoneNumberInput
          label="Phone number"
          bind:value={supplier.phoneNumber}
        />
        <EmailInput label="Email" bind:value={supplier.email} />
      {/if}
    </Card>
  {/each}
  <Button style="width: 500px;" kind="tertiary" on:click={new_supplier}
    >+ Add</Button
  >
</div>

<style lang="scss">
  .suppliers {
    width: 100%;
    padding: 40px;
    display: flex;
    align-items: center;
    flex-direction: column-reverse;
    gap: 40px;
  }
</style>
