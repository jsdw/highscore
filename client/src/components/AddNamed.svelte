<script lang="ts">
    import Modal from './Modal.svelte'
    import Button from './Button.svelte'

    export let title: string
    export let description: string
    export let label: string = "value"
    export let on_cancel: () => void
    export let on_try_add: (name: string) => void
    export let type: "text" | "password" | "number" = "text"

    let name = ""

    let input_el: HTMLInputElement | null = null
    $: focus_input_el(input_el)

    function focus_input_el(input_el: HTMLInputElement | null) {
        if (input_el) input_el.focus()
    }

    function input_keypress(e: KeyboardEvent) {
        if (e.key === "Enter") {
            on_try_add(name)
        }
    }

</script>

<Modal title={title} on_close={on_cancel}>
    <div class="row">
        <label for={label}>{description}</label>
        {#if type === "password"}
            <input bind:this={input_el} type="password" name={label} bind:value={name} on:keypress={input_keypress}/>
        {:else if type === "text"}
            <input bind:this={input_el} name={label} bind:value={name} on:keypress={input_keypress}/>
        {:else if type === "number"}
            <input bind:this={input_el} type="number" name={label} bind:value={name} on:keypress={input_keypress}/>
        {/if}
    </div>
    <div class="buttons">
        <Button disabled={!name} on_click={() => on_try_add(name)}>Save</Button>
        <Button color="red" on_click={on_cancel}>Cancel</Button>
    </div>
</Modal>

<style>
    label {
        margin-bottom: 0.25em;
    }
    .row {
        margin-top: 1em;
        margin-bottom: 1em;
    }
    .buttons {
        display: flex;
        justify-content: space-between;
    }
</style>