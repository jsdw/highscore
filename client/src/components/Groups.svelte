<script lang="ts">
    import type { api as apiTypes } from '../api'
    import { api } from '../api'
    import { on_last_changed } from '../stores'
    import AddNamed from './AddNamed.svelte'
    import Button from './Button.svelte'

    export let on_select_group: (id: string) => void

    let groups: apiTypes.GroupsOutput = []
    let showing_add_modal = false
    let loading = true

    on_last_changed(get_details)

    async function get_details() {
        const g = await api.groups()
        groups = g
        loading = false
    }

    function add_group(name: string) {
        hide_add_modal()
        api.upsert_group({ name }).finally(get_details)
    }
    function show_add_modal() {
        showing_add_modal = true
    }
    function hide_add_modal() {
        showing_add_modal = false
    }

    function show_group(id: string) {
        on_select_group(id)
    }
</script>

{#if !loading}
    <div class="container">
        <div class="inner">
            {#each groups as group (group.id) }
                <div class="group" on:click={() => show_group(group.id)}>
                    <span>{group.name}</span>
                    <Button>Show</Button>
                </div>
            {/each}
            <div class="buttons">
                <Button on_click={show_add_modal}>Add group</Button>
            </div>
        </div>
    </div>
{/if}

{#if showing_add_modal}
    <AddNamed
        title='Add Group'
        description='Name'
        on_try_add={add_group}
        on_cancel={hide_add_modal}
    />
{/if}

<style>
    .container {
        display: flex;
        justify-content: center;
        margin: 1em;
    }
    .inner {
        flex-grow: 1;
    }
    .group {
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
    .group > span {
        margin-right: 1em;
    }
</style>