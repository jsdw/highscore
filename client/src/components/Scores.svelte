<script lang="ts">
    import type { api as apiTypes } from '../api'
    import { api } from '../api'
    import Confirm from './Confirm.svelte'
    import AddNamed from './AddNamed.svelte'
    import Button from './Button.svelte'
    import Link from './Link.svelte'

    export let scorable_id: string
    export let current_user: string

    let scores: apiTypes.ScoresOutput = []
    let name: string = ""
    let showing_add_score_modal = false
    let showing_confirm_delete_modal = false
    let score_id_to_delete = ""
    let loading = true

    get_details()

    async function get_details() {
        const res = await api.scores({ scorable_id })
        const scorable = await api.get_scorable({ id: scorable_id })
        scores = res
        name = scorable.name
        loading = false
    }

    function pretty_print_iso_date(iso_date: string) {
        const d = new Date(iso_date)
        return `${pad_to(2,d.getUTCDate())}/${pad_to(2,d.getUTCMonth())}/${d.getUTCFullYear()}`
    }

    function pad_to(len: number, n: number) {
        let s = n.toString()
        while (s.length < len) { s = "0" + s }
        return s
    }

    function show_add_score_modal() {
        showing_add_score_modal = true
    }
    function hide_add_score_modal() {
        showing_add_score_modal = false
    }
    async function add_score(value: string) {
        showing_add_score_modal = false
        await api.upsert_score({ scorable_id, value: Number(value) })
        get_details()
    }

    function show_confirm_delete_modal(score_id: string) {
        showing_confirm_delete_modal = true
        score_id_to_delete = score_id
    }
    function hide_confirm_delete_modal() {
        showing_confirm_delete_modal = false
    }
    async function delete_score() {
        showing_confirm_delete_modal = false
        await api.delete_score({ id: score_id_to_delete })
        get_details()
    }
</script>

{#if !loading}
    <div class="container">
        <h1>{name}</h1>
        <div class="buttons">
            <Button on_click={show_add_score_modal}>Add Score</Button>
        </div>
        {#if scores.length}
            <table>
                <tr>
                    <th>Name</th>
                    <th>Score</th>
                    <th>Date</th>
                    <th></th>
                </tr>
                {#each scores as score (score.id) }
                    <tr>
                        <td>{score.username}</td>
                        <td>{score.value}</td>
                        <td>{pretty_print_iso_date(score.date)}</td>
                        <td>
                            {#if score.username === current_user}
                                <Link on_click={() => show_confirm_delete_modal(score.id)}>delete</Link>
                            {/if}
                        </td>
                    </tr>
                {/each}
            </table>
        {:else}
            No scores have been set
        {/if}
    </div>
{/if}

{#if showing_add_score_modal}
    <AddNamed
        title="Add Score"
        label="Score"
        type="number"
        on_cancel={hide_add_score_modal}
        on_try_add={add_score}
    />
{/if}

{#if showing_confirm_delete_modal}
    <Confirm
        title="Delete Score"
        description="Are you sure you'd like to delete the score. This cannot be undone."
        on_cancel={hide_confirm_delete_modal}
        on_confirm={delete_score}
    />
{/if}

<style>
    h1 {
        text-align: center;
    }
    .container {
        margin: 1em;
        display: flex;
        align-items: center;
        flex-direction: column;
    }
    .buttons {
        margin-bottom: 1em;
    }
    table {
        width: 700px;
        max-width: calc(100% - 2em);
    }
    td, th {
        text-align: center;
        padding: 0.25em 0.5em;
    }
</style>