<script lang="ts">
    export let color: "blue" | "red" = "blue"
    export let disabled = false
    export let on_click: () => void = () => null

    let button_class = `${color === "blue" ? "blue" : "red"}`
</script>

<button disabled={disabled} class={button_class} on:click={on_click}>
    <slot/>
</button>

<style>
    button {
        font-family: inherit;
        font-size: inherit;
        -webkit-padding: 0.4em 0;
        padding: 0.4em;
        margin: 0 0 0 0;
        box-sizing: border-box;
        border-radius: var(--border);
        outline: none;
        padding: 0.4em 1em;
        cursor: pointer;
        box-shadow: 0px 3px 5px 0px rgba(0,0,0,0.2);

        color: var(--white-light2);
        border: 1px solid var(--blue-dark2);
        background-color: var(--blue);
    }
    button:disabled {
        color: var(--white-dark2);
        opacity: 0.8;
    }
    button:not(:disabled):active {
        background-color: var(--blue-light2);
    }
    button:focus {
        border-color: var(--blue-light2);
    }
    button:hover {
        background-color: var(--blue-light1);
    }

    button.red {
        background-color: var(--red);
        border-color: var(--red-dark2);
    }
    button.red:disabled {
        color: var(--red-dark2);
    }
    button.red:not(:disabled):active {
        background-color: var(--red-light2);
    }
    button.red:focus {
        border-color: var(--red-light2);
    }
    button.red:hover {
        background-color: var(--red-light1);
    }
</style>