<script async script lang="ts">
  import type {
    ProductNames,
    PendingOrder,
    Product,
    ReceivedOrder,
  } from "../../lib/types";
  import GroupBox from "../../lib/GroupBox.svelte";
  import Card from "../../lib/Card.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button } from "carbon-components-svelte";
  import NumberInput from "../../lib/NumberInput.svelte";
  import Search from "../../lib/Search.svelte";

  export let orders: PendingOrder[];
  export let receivedOrders: ReceivedOrder[];
  export let firstProduct: number;
  export let getProduct: (id: number) => Product;
  export let open: boolean[] = orders.map(() => false);

  async function remove(index: number) {
    let order = orders.splice(index, 1)[0];
    console.log(order.id);
    await invoke("remove_pending_order", { id: order.id })
      .then(() => (orders = orders))
      .catch((e) => console.log(e));
    orders = orders;
  }
  async function new_order() {
    await invoke("new_pending_order", { productId: firstProduct })
      .then((order: PendingOrder) => {
        order.getProduct = () => getProduct(order.product);
        orders = [
          ...orders,
          {
            ...order,
          },
        ];
      })
      .catch((e) => console.log(e));
  }

  let editing: boolean[] = orders.map((order) => false);
  function edit(index: number) {
    editing[index] = true;
  }
  async function save(index: number) {
    await invoke("save_pending_order", { order: orders[index] })
      .then(() => (editing[index] = false))
      .catch((e) => console.log(e));
    orders = orders;
    orders[index] = orders[index];
    orders[index].getProduct = () => getProduct(orders[index].product);

    console.log(orders[index].getProduct().name, orders[index].product);
  }
  async function markAsReceived(index: number) {
    let today = new Date();
    let dd = today.getDate().toString();
    let mm = (today.getMonth() + 1).toString(); //January is 0!
    let yyyy = today.getFullYear().toString();

    if (dd.length === 1) {
      dd = "0" + dd;
    }
    if (mm.length === 1) {
      mm = "0" + mm;
    }

    let date = mm + "/" + dd + "/" + yyyy;
    await invoke("mark_order_received", {
      order: orders[index],
      date,
      actuallyReceived: orders[index].amount,
      damaged: 0,
    })
      .then(
        (order: ReceivedOrder) => (
          orders.splice(index, 1),
          (orders = orders),
          (order.getProduct = function () {
            return getProduct(this.product_id);
          }),
          (receivedOrders = [...receivedOrders, order]),
          (receivedOrders = receivedOrders)
        )
      )
      .catch((e) => console.log(e));
  }
  let headers = [
    { key: "name", value: "Name" },
    { key: "upc", value: "UPC" },
    { key: "id", value: "ID" },
  ];
  export let rows: ProductNames[] = [];
  (async () => {
    await invoke("product_names").then(
      (product: ProductNames[]) => (rows = product)
    );
  })();
  // comment

  export let selectedRowIds: any[][] = orders.map(() => []);
  export let text = "";
  export let search: () => void = async () => {
    console.log("search");
    await invoke("sort_products", { productNames: rows, search: text })
      .then((res: ProductNames[]) => (console.log(res), (rows = res)))
      .catch((e) => console.log(e));
  };
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
        <GroupBox label="amount" value={order.amount} />
        <Button kind="primary" on:click={() => markAsReceived(index)}
          >Mark as Received</Button
        >
      {:else}
        <div>
          <div class="products">
            {order.getProduct() ? order.getProduct().name : "(removed)"}
            <Button on:click={() => (open[index] = true)}>Choose Product</Button
            >
            {#if open[index] == true}
              <Search
                bind:open={open[index]}
                bind:text
                bind:rows
                {search}
                okay={() => (orders[index].product = selectedRowIds[index][0])}
                bind:selectedRowIds={selectedRowIds[index]}
                {headers}
              />
            {/if}
          </div>
        </div>
        <NumberInput label="Amount" bind:value={order.amount} />
      {/if}
    </Card>
  {/each}
  <Button style="width: 500px;" kind="tertiary" on:click={new_order}
    >+ Add</Button
  >
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
  .products {
    display: flex;
    flex-direction: column;
  }
</style>
