<script lang="ts">
    import { api } from '../api'

    export let on_login: (username: string) => void

    let username = ""
    let password = ""
    let is_invalid = false

    function login() {
        api.login({
            username,
            password
        }).then(_ => {
            on_login(username)
        }).catch(_ => {
            is_invalid = true
        })
    }

    function input_keypress(event: KeyboardEvent) {
        if (event.key === 'Enter') {
            login()
        }
    }
</script>

<div class="container">
    <div class="login-box">
        <label for="username">Username</label>
        <input name="username" bind:value={username} on:keypress={input_keypress}/>

        <label for="password">Password</label>
        <input name="password" type="password" bind:value={password} on:keypress={input_keypress}/>

        <button on:click={login}>Login</button>
        {#if is_invalid}
            <div class="invalid">
                Incorrect details provided; try again
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        height: 100%;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    input {
        display: block;
    }
</style>