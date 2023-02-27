<script lang="ts">
  import type { Product } from "src/lib/types";
  import GroupBox from "../../lib/GroupBox.svelte";
  import Card from "../../lib/Card.svelte";

  export let lowInventory: Product[];
</script>

<div class="dashboard">
  {#each lowInventory as product (product.id)}
    <Card>
      <GroupBox label="name" value={product.name} />
      <GroupBox label="upc" value={product.upc} />
      <GroupBox label="amount" value={product.amount} />
      <GroupBox label="buy level" value={product.buyLevel} />
      <GroupBox
        label="product ratio"
        value={(product.amount / product.buyLevel).toFixed(2)}
      />
    </Card>
  {/each}
</div>

<style lang="scss">
  .dashboard {
    width: 100%;
    padding: 40px;
    display: flex;
    align-items: center;
    flex-direction: column-reverse;
    gap: 40px;
  }
</style>
