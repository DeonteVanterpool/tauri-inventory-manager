<script lang="ts">
  import { Button, DataTable, Search } from "carbon-components-svelte";
  import Modal from "./Modal.svelte";
  export let open: boolean;
  export let okay: () => void;
  export let headers: { key: string; value: string }[];
  export let rows: any[];
  export let selectedRowIds: any[] = [];
  export let text: string = "";
  export let search: () => void;
  export let radio = true;
  export let batchSelection = false;
</script>

<div class="search">
  <Search bind:value={text} on:input={search} placeholder="Search..." />
  <Modal bind:open {okay}>
    <DataTable
      {radio}
      {batchSelection}
      bind:selectedRowIds
      {headers}
      bind:rows
    />
  </Modal>
</div>

<style>
  .search {
    position: fixed;
    width: 50vw;
    height: 50vh;
    background: white;
    top: 0;
    right: 0;
    z-index: 100;
  }
</style>
