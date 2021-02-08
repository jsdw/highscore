<script lang="ts">
    import Modal from './Modal.svelte'
    import Button from './Button.svelte'
    import Confirm from './Confirm.svelte'

    export let title: string
    export let description: string
    export let label: string = "value"
    export let current_name: string
    export let on_cancel: () => void
    export let on_rename: (name: string) => void
    export let on_delete: () => void

    let name = current_name

    function input_keypress(e: KeyboardEvent) {
        if (e.key === "Enter") {
            on_rename(name)
        }
    }

    let showing_delete_confirm = false
    function show_delete_confirm() {
        showing_delete_confirm = true
    }
    function hide_delete_confirm() {
        showing_delete_confirm = false
    }
    function confirm_delete() {
        showing_delete_confirm = false
        on_delete()
    }

</script>

<Modal title={title} on_close={on_cancel} width={350}>
    <div class="row">
        <label for={label}>{description}</label>
        <input name={label} bind:value={name} on:keypress={input_keypress}/>
    </div>
    <div class="delete">
        <span>Danger zone. Deleting cannot be undone.</span>
        <Button color="red" disabled={!name} on_click={() => show_delete_confirm()}>Delete</Button>
    </div>
    <div class="buttons">
        <Button color="red" disabled={!name} on_click={() => on_rename(name)}>Save</Button>
        <Button on_click={on_cancel}>Cancel</Button>
    </div>
</Modal>

{#if showing_delete_confirm}
    <Confirm
        title="Confirm Delete"
        description="Are you sure you wish to delete this?"
        on_confirm={confirm_delete}
        on_cancel={hide_delete_confirm}
    />
{/if}

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
    .delete {
        padding: 15px;
        background-color: var(--red-dark2);
        margin-top: 1em;
        margin-bottom: 1em;
        border-radius: var(--border);
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    .delete > span {
        margin-right: 1em;
    }
</style>