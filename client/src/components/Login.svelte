<script lang="ts">
    import { api } from '../api'
    import Button from './Button.svelte'

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
        <h1>Highscore</h1>

        <div class="row">
            <label for="username">Username</label>
            <input autocomplete="username" autocapitalize="none" name="username" bind:value={username} on:keypress={input_keypress}/>
        </div>

        <div class="row">
            <label for="password">Password</label>
            <input name="password" type="password" bind:value={password} on:keypress={input_keypress}/>
        </div>

        <Button on_click={login}>Login</Button>
        {#if is_invalid}
            <div class="invalid">
                Incorrect details provided; try again
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        position: absolute;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: var(--charcoal-dark2);
    }
    .login-box {
        background-color: var(--charcoal);
        padding: 20px;
        margin: 10px;
        border-radius: 3px;
        box-shadow: 0px 3px 11px 1px rgba(0,0,0,0.5);
        width: 270px;
    }
    h1 {
        margin: 0;
        padding-bottom: 15px;
        border-bottom: 1px solid var(--charcoal-dark2);
        margin-bottom: 20px;
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