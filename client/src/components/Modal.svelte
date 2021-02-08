<script lang="ts">
    import Portal from './Portal.svelte'
    import { onMount } from 'svelte'

    export let on_close: () => void = () => null
    export let width: number = 0
    export let title: string
    onMount(() => {
        const handleEscape = (e: KeyboardEvent) => {
            if(e.key === 'Escape') on_close()
        }
        document.body.addEventListener('keydown', handleEscape)
        return () => document.body.removeEventListener('keydown', handleEscape)
    })
</script>

<Portal>
    <div class="modal-background" on:click={on_close}>
        <div class="modal-foreground" on:click|stopPropagation style={`width: ${width || 'nope'}px`}>
            <div class="title">
                <h2>{title}</h2>
            </div>
            <div class="body">
                <slot/>
            </div>
        </div>
    </div>
</Portal>

<style>
    .modal-background {
        position: fixed;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
        background-color: rgba(0,0,0,0.3);
        display: flex;
        align-items: center;
        justify-content: center;
    }
    .modal-foreground {
        display: flex;
        flex-direction: column;
        max-width: calc(100% - 20px);
        max-height: calc(100% - 20px);
        overflow: auto;
        background-color: var(--charcoal-light2);
        border-radius: 3px;
        box-shadow: 0px 3px 10px 2px rgba(0,0,0,0.3);
        padding: 15px;
    }
    .title h2 {
        margin-top: 0;
        margin-bottom: 0;
        text-overflow: ellipsis;
        overflow: hidden;
        white-space: nowrap;
    }
    .title {
        padding-bottom: 15px;
        border-bottom: 1px solid rgba(0,0,0,0.2);
    }
    .body {
        flex-grow: 1;
        overflow: auto;
        position: relative;
    }
</style>