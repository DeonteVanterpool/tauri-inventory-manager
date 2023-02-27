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
    margin-top: 100px;
    top: 0;
    font-weight: bold;
    height: 100px;
    font-size: 2em;
    text-align: center;
    box-shadow: 2px 2px 5px rgb(224, 224, 224);
    border-bottom: 5px solid rgb(224, 224, 224);
    display: flex;
    align-items: center;
    width: calc(100% - 300px);
    text-transform: capitalize;
    display: flex;
    .text {
      text-align: center;
      justify-self: center;
      margin: auto auto;
    }
  }
</style>
