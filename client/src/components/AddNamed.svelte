<script lang="ts">
    import Modal from './Modal.svelte'
    import Button from './Button.svelte'

    export let title: string
    export let description: string
    export let label: string = "value"
    export let on_cancel: () => void
    export let on_try_add: (name: string) => void
    export let type: "text" | "password" | "number" = "text"
    export let width: number = 325
    export let confirm_text = "Save"
    export let cancel_text = "Cancel"

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

<Modal title={title} on_close={on_cancel} width={width}>
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
        <Button color="red" disabled={!name} on_click={() => on_try_add(name)}>{confirm_text}</Button>
        <Button on_click={on_cancel}>{cancel_text}</Button>
    </div>
</Modal>

<style>
    label {
        margin-bottom: 0.5em;
    }
    input {
        width: 100%;
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