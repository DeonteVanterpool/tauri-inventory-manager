<script lang="ts">
  import GroupBox from "../../lib/GroupBox.svelte";
  import Card from "../../lib/Card.svelte";
  import Pending from "./Pending.svelte";
  import Received from "./Received.svelte";
  import type { PendingOrder, Product, ReceivedOrder } from "src/lib/types";

  export let pendingOrders: PendingOrder[];
  export let receivedOrders: ReceivedOrder[];
  export let orderIndex: number = 0;
  export let firstProduct: number;
  export let getProduct: (id: number) => Product;
</script>

<div class="orders">
  {#if orderIndex == 0}
    <Pending
      bind:orders={pendingOrders}
      {firstProduct}
      {getProduct}
      bind:receivedOrders
    />
  {:else}
    <Received bind:orders={receivedOrders} />
  {/if}
</div>

<style lang="scss">
  .orders {
    display: flex;
    width: 100%;
    flex-direction: column;
  }
</style>
