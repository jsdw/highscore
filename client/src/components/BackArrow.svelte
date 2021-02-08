<script lang="ts">
    export let on_click: () => void
</script>

<span class="back-arrow" on:click={() => on_click()}>
    &#8592;
</span>

<style>
    .back-arrow {
        cursor: pointer;
        color: var(--blue);
        font-size: 30px;
        line-height: 15px;
    }
    .back-arrow:hover {
        color: var(--blue-light2);
    }
</style>