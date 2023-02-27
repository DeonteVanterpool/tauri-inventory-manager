<script lang="ts">
  import "carbon-components-svelte/css/white.css";
  import { onMount } from "svelte";
  export let sideBarItems: string[];
  export let selected: number = 0;

  onMount(() => select(0));

  function unselectAllItems() {
    let element = document.getElementsByClassName("sidebar-item");
    for (let index = 0; index < element.length; index++) {
      element[index].setAttribute("style", "");
    }
  }

  function select(index: number) {
    unselectAllItems();

    let items = document.getElementsByClassName("sidebar-item");

    items[index].setAttribute(
      "style",
      "background: linear-gradient(90deg, rgb(237, 237, 255) 98%, blue 98%);"
    );

    selected = index;
  }
</script>

<div class="sidebar">
  <div class="logo">InventoryManager</div>
  {#each sideBarItems as sideBarItem, index}
    <button on:click={() => select(index)} class="sidebar-item">
      {sideBarItem}
    </button>
  {/each}
</div>

<style lang="scss">
  .sidebar {
    background-color: white;
    position: fixed;
    width: 300px;
    border-right: 5px solid rgb(224, 224, 224);
    justify-self: left;
    height: 100vw;
    top: 0;
    text-align: left;
    display: flex;
    flex-direction: column;
    justify-content: start;
    button.sidebar-item {
      background-color: transparent;
      border: none;
      height: 50px;
      box-shadow: none;
      font-weight: normal;
      padding-left: 30px;
      padding-right: 30px;
      text-transform: capitalize;
      display: flex;
      align-items: center;
      color: black;
      font-size: 1.2em;
      &:hover {
        background-color: rgb(237, 237, 255);
      }
    }
    .logo {
      height: 100px;
      padding: 50px;
      font-weight: bold;
      font-size: 1.5em;
      align-self: center;
    }
  }
</style>
