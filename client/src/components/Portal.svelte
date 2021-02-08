<script lang="ts">
    import { onMount } from 'svelte'

    let ref: HTMLDivElement
    onMount(() => {
        // Svelte will clean this up when we're done,
        // so we don't need to ourselves
        document.body.appendChild(ref)
    })
</script>

<div bind:this={ref}>
    <slot/>
</div>