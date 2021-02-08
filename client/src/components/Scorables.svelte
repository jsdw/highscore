<script lang="ts">
    import type { api as apiTypes } from '../api'
    import { api } from '../api'
    import BackArrow from './BackArrow.svelte'
    import AddNamed from './AddNamed.svelte'
    import EditNamed from './EditNamed.svelte'
    import Button from './Button.svelte'
    import Link from './Link.svelte'

    export let on_select_scorable: (id: string, group_id: string) => void
    export let on_go_back: (group_id: string) => void
    export let group_id: string

    let scorables: apiTypes.ScorablesInGroupOutput = []
    let group_name: string = ""
    let showing_add_modal = false
    let showing_edit_group_modal = false
    let loading = true

    get_details()

    async function get_details() {
        const group = await api.get_group({ id: group_id }).catch(() => undefined)
        const res = group ? await api.scorables_in_group({ group_id }) : []
        scorables = res
        group_name = group ? group.name : ""
        loading = false
    }

    async function add_scorable(name: string) {
        hide_add_modal()
        await api.upsert_scorable({ name, group_id })
        get_details()
    }

    function show_add_modal() {
        showing_add_modal = true
    }
    function hide_add_modal() {
        showing_add_modal = false
    }

    function show_edit_group_modal() {
        showing_edit_group_modal = true
    }
    function hide_edit_group_modal() {
        showing_edit_group_modal = false
    }
    function rename_group(new_name: string) {
        hide_edit_group_modal()
        api.upsert_group({ id: group_id, name: new_name }).finally(get_details)
    }
    function delete_group() {
        hide_edit_group_modal()
        api.delete_group({ id: group_id }).finally(get_details)
    }

    function show_scorable(id: string) {
        on_select_scorable(id, group_id)
    }

</script>

{#if !loading && group_name}
    <div class="container">
        <BackArrow on_click={() => on_go_back(group_id)}/>
        <h1>{group_name}</h1>
        <div class="settings">
            (<Link on_click={show_edit_group_modal}>edit group</Link>)
        </div>
        <div class="inner">
            {#each scorables as scorable (scorable.id) }
                <div class="scorable" on:click={() => show_scorable(scorable.id)}>
                    <span>{scorable.name}</span>
                    <Button>Show</Button>
                </div>
            {:else}
                <div class="no-scores">
                    No scores have been set in thie group.
                </div>
            {/each}
            <div class="create">
                <Button on_click={show_add_modal}>Add scores</Button>
            </div>
        </div>
    </div>
{:else if !loading && !group_name}
    <h1>Group not found</h1>
{/if}

{#if showing_add_modal}
    <AddNamed
        title='Add Scorable'
        description='Name'
        on_try_add={add_scorable}
        on_cancel={hide_add_modal}
    />
{/if}

{#if showing_edit_group_modal}
    <EditNamed
        title='Edit Group'
        description='Name'
        current_name={group_name}
        on_rename={rename_group}
        on_delete={delete_group}
        on_cancel={hide_edit_group_modal}
    />
{/if}

<style>
    h1 {
        text-align: center;
        margin-bottom: 4px;
        margin-top: 0;
    }
    .settings {
        text-align: center;
        margin-bottom: 20px;
    }
    .container {
        margin: 1em;
    }
    .inner {
        flex-grow: 1;
    }
    .no-scores {
        margin-bottom: 1em;
        background-color: var(--charcoal-light1);
        padding: 1em;
        width: 100%;
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
    .create {
        margin-top: 1em;
    }
</style>