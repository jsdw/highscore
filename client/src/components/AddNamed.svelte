<script lang="ts">
    import Modal from './Modal.svelte'
    import Button from './Button.svelte'

    export let title: string
    export let label: string
    export let on_cancel: () => void
    export let on_try_add: (name: string) => void

    let name = ""

    function input_keypress(e: KeyboardEvent) {
        if (e.key === "Enter") {
            on_try_add(name)
        }
    }

</script>

<Modal title={title} on_close={on_cancel}>
    <div class="row">
        <label for="name">{label}</label>
        <input name="name" bind:value={name} on:keypress={input_keypress}/>
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