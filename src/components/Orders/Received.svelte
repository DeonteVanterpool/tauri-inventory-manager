<script lang="ts">
  import type { ReceivedOrder } from "src/lib/types";
  import GroupBox from "../../lib/GroupBox.svelte";
  import Card from "../../lib/Card.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, DatePicker, TextInput } from "carbon-components-svelte";
  import NumberInput from "../../lib/NumberInput.svelte";

  export let orders: ReceivedOrder[];

  async function remove(index: number) {
    let order = orders.splice(index, 1)[0];
    console.log(order.id);
    await invoke("remove_received_order", { id: order.id })
      .then(() => (orders = orders))
      .catch((e) => console.log(e));
    orders = orders;
  }

  let editing: boolean[] = orders.map((order) => false);
  function edit(index: number) {
    editing[index] = true;
  }
  // comment
  async function save(index: number) {
    await invoke("save_received_order", { order: orders[index] })
      .then(() => (editing[index] = false))
      .catch((e) => console.log(e));
    orders[index] = orders[index];
    orders[index].getProduct = orders[index].getProduct;
  }
</script>

<div class="orders">
  {#each orders as order, index (order.id)}
    <Card
      editable={true}
      edit={() => edit(index)}
      save={() => save(index)}
      remove={() => remove(index)}
    >
      {#if !editing[index]}
        <GroupBox
          label="product"
          value={order.getProduct() ? order.getProduct().name : "(removed)"}
        />
        <GroupBox label="amount ordered" value={order.gross_amount} />
        <GroupBox label="amount received" value={order.actually_received} />
        <GroupBox label="date received" value={order.received} />
        <GroupBox label="amount damaged" value={order.damaged} />
      {:else}
        <div>
          <div>
            {order.getProduct() ? order.getProduct().name : "removed"}
          </div>
        </div>
        <NumberInput label="Amount Ordered" bind:value={order.gross_amount} />
        <NumberInput
          label="Amount Received"
          bind:value={order.actually_received}
        />
        <TextInput
          label="Received Date"
          bind:value={order.received}
          placeholder="mm/dd/yyyy"
        />
        <NumberInput label="Amount Damaged" bind:value={order.damaged} />
      {/if}
    </Card>
  {/each}
</div>

<style lang="scss">
  .orders {
    width: 100%;
    padding: 40px;
    display: flex;
    align-items: center;
    flex-direction: column-reverse;
    gap: 40px;
  }
</style>
