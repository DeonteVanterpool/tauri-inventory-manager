<script lang="ts">
  export let headerText: string = "";
</script>

<div class="header">
  <div class="text">{headerText}</div>
</div>

<style lang="scss">
  .header {
    background-color: white;
    position: fixed;
    margin-left: 300px;
    text-indent: 1.5em;
    font-weight: bold;
    height: 100px;
    font-size: 2em;
    text-align: left;
    top: 0;
    border-bottom: 5px solid rgb(224, 224, 224);
    display: flex;
    align-items: center;
    width: calc(100% - 300px);
    text-transform: capitalize;
    .text {
      justify-self: start;
    }
  }
</style>
