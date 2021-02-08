<script lang="ts">
    import type { api as apiTypes } from '../api'
    import { api } from '../api'
    import AddNamed from './AddNamed.svelte'
    import Button from './Button.svelte'

    export let on_select_scorable: (id: string) => void
    export let group_id: string

    let scorables: apiTypes.ScorablesInGroupOutput = []
    let group_name: string = ""
    let show_add_modal = false

    get_details()

    async function get_details() {
        const res = await api.scorables_in_group({ group_id })
        const group = await api.get_group({ id: group_id })
        scorables = res
        group_name = group.name
    }

    function add_scorable(name: string) {
        hide_modal()
        api.upsert_group({ name }).finally(get_details)
    }

    function show_modal() {
        show_add_modal = true
    }
    function hide_modal() {
        show_add_modal = false
    }

    function show_scorable(id: string) {
        on_select_scorable(id)
    }

</script>

<h1>{group_name}</h1>
<div class="container">
    <div class="inner">
        {#each scorables as scorable (scorable.id) }
            <div class="scorable" on:click={() => show_scorable(scorable.id)}>
                <span>{scorable.name}</span>
                <Button>Show</Button>
            </div>
        {/each}
        <div class="create">
            <Button on_click={show_modal}>Add group</Button>
        </div>
    </div>
</div>
{#if show_add_modal}
    <AddNamed
        title='Add Scorable'
        label='Name'
        on_try_add={add_scorable}
        on_cancel={hide_modal}
    />
{/if}

<style>
    h1 {
        margin: 1em;
        text-align: center;
    }
    .container {
        display: flex;
        justify-content: center;
        margin: 1em;
    }
    .inner {
        flex-grow: 1;
    }
    .scorable {
        cursor: pointer;
        user-select: none;
        margin-bottom: 1em;
        background-color: var(--charcoal-light1);
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1em;
        width: 100%;
    }
    .scorable > span {
        margin-right: 1em;
    }
</style>