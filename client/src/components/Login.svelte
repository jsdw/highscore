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
        <div class="row">
            <label for="username">Username</label>
            <input name="username" bind:value={username} on:keypress={input_keypress}/>
        </div>

        <div class="row">
            <label for="password">Password</label>
            <input name="password" type="password" bind:value={password} on:keypress={input_keypress}/>
        </div>

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
        background-color: var(--charcoal-dark2);
    }
    .login-box {
        background-color: var(--charcoal);
        padding: 1.25em;
        margin: 10px;
        border-radius: 3px;
        box-shadow: 0px 3px 11px 1px rgba(0,0,0,0.5);
        width: 270px;
    }
    .row {
        margin-bottom: 1em;
    }
    label {
        margin-bottom: 0.25em;
    }
    input {
        display: block;
        width: 100%;
    }
    .invalid {
        padding: 1em;
        border-radius: var(--border);
        background-color: var(--red);
        margin-top: 1em;
    }
</style>