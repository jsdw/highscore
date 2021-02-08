<script lang="ts">
    import Modal from './Modal.svelte'
    import Button from './Button.svelte'

    export let title: string
    export let description: string
    export let on_cancel: () => void
    export let on_confirm: () => void

</script>

<Modal title={title} on_close={on_cancel} width={300}>
    <div class="description">
        {description}
    </div>
    <div class="buttons">
        <Button color="red" on_click={() => on_confirm()}>Yes</Button>
        <Button on_click={on_cancel}>No</Button>
    </div>
</Modal>

<style>
    .description {
        margin-bottom: 1em;
        margin-top: 1em;
    }
    .buttons {
        display: flex;
        justify-content: space-between;
    }
</style>