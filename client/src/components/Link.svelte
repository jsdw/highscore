<script lang="ts">
    export let on_click: () => void
</script>

<span on:click={() => on_click()}>
    <slot/>
</span>

<style>
    span {
        color: var(--blue-light1);
        text-decoration: underline;
        cursor: pointer;
    }

    span:hover {
        color: var(--blue-light2);
    }
</style>