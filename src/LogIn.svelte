<script lang="ts">
  import { TextInput, PasswordInput, Button } from "carbon-components-svelte";
  export let logIn: (username: string, password: string) => Promise<void>;
  export let name: string = "";
  export let password: string = "";
</script>

<div class="login">
  <TextInput
    bind:value={name}
    labelText="Username/email"
    placeholder="Enter your username or your email for login..."
  />
  <PasswordInput
    bind:value={password}
    labelText="Password"
    placeholder="Enter your login password..."
  />
  <Button on:click={async () => await logIn(name, password)}>Log In</Button>
</div>

<style>
  .login {
    display: flex;
    flex-direction: column;
    gap: 50px;
    padding: 100px;
  }
</style>
