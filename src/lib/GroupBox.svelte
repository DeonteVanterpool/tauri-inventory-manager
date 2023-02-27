<script lang="ts">
  import { Button, ContentSwitcher, Switch } from "carbon-components-svelte";

  export let label: string;
  export let value: any;
</script>

<div class="group-box">
  <div class="label">{label}:</div>
  <div class="value">{value}</div>
</div>

<style lang="scss">
  .group-box {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .label {
    font-weight: bold;
    text-transform: capitalize;
  }
</style>
