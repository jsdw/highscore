<script lang="ts">
    import type { api as apiTypes } from '../api'
    import { api } from '../api'
    import { on_last_changed } from '../stores'
    import BackArrow from './BackArrow.svelte'
    import Confirm from './Confirm.svelte'
    import AddNamed from './AddNamed.svelte'
    import EditNamed from './EditNamed.svelte'
    import Button from './Button.svelte'
    import Link from './Link.svelte'

    export let on_go_back: (group_id: string) => void
    export let scorable_id: string
    export let group_id: string
    export let current_user: string

    let scores: apiTypes.ScoresOutput = []
    let name: string = ""
    let showing_add_score_modal = false
    let showing_confirm_delete_modal = false
    let score_id_to_delete = ""
    let loading = true
    $: can_delete_some_scores = scores.some(s => s.username === current_user)

    on_last_changed(get_details)

    async function get_details() {
        const scorable = await api.get_scorable({ id: scorable_id }).catch(() => undefined)
        const res = scorable ? await api.scores({ scorable_id }) : []
        scores = res
        name = scorable ? scorable.name : ""
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

    let showing_edit_scorable_modal = false
    function show_edit_scorable_modal() {
        showing_edit_scorable_modal = true
    }
    function hide_edit_scorable_modal() {
        showing_edit_scorable_modal = false
    }
    function rename_scorable(new_name: string) {
        hide_edit_scorable_modal()
        api.upsert_scorable({ id: scorable_id, group_id, name: new_name }).finally(get_details)
    }
    function delete_scorable() {
        hide_edit_scorable_modal()
        api.delete_scorable({ id: scorable_id }).finally(get_details)
    }
</script>

<div class="back-container">
<BackArrow on_click={() => on_go_back(group_id)}/>
</div>
<div class="scores-container">
    {#if !loading && name}
        <h1>{name}</h1>
        <div class="settings">
            (<Link on_click={show_edit_scorable_modal}>edit scorable</Link>)
        </div>
        <div class="buttons">
            <Button on_click={show_add_score_modal}>Add Score</Button>
        </div>
        {#if scores.length}
            <table>
                <tr>
                    <th class="padding"></th>
                    <th class="name">Name</th>
                    <th class="score">Score</th>
                    <th class="date">Date</th>
                    {#if can_delete_some_scores}
                        <th class="delete"></th>
                    {/if}
                    <th class="padding"></th>
                </tr>
                {#each scores as score (score.id) }
                    <tr>
                        <td class="padding"></td>
                        <td class="name">{score.username}</td>
                        <td class="score">{score.value}</td>
                        <td class="date">{pretty_print_iso_date(score.date)}</td>
                        {#if can_delete_some_scores}
                            <td class="delete">
                                {#if score.username === current_user}
                                    <div class="delete-icon" on:click={() => show_confirm_delete_modal(score.id)}>&#215;</div>
                                {/if}
                            </td>
                        {/if}
                        <td class="padding"></td>
                    </tr>
                {/each}
            </table>
        {:else}
            No scores have been set
        {/if}
    {:else if !loading && !name}
        <h1>Scorable not found</h1>
    {/if}
</div>

{#if showing_add_score_modal}
    <AddNamed
        title="Add Score"
        description="Score"
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

{#if showing_edit_scorable_modal}
    <EditNamed
        title='Edit Scorable'
        description='Name'
        current_name={name}
        on_rename={rename_scorable}
        on_delete={delete_scorable}
        on_cancel={hide_edit_scorable_modal}
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
    .back-container {
        margin-top: 1em;
        margin-left: 1em;
    }
    .scores-container {
        margin-bottom: 1em;
        display: flex;
        align-items: center;
        flex-direction: column;
        overflow-x: auto;
    }
    .buttons {
        margin-bottom: 1em;
    }
    table {
        width: 100%;
        border-collapse: collapse;
    }
    td, th {
        text-align: center;
        padding: 0.75em 0.5em;
    }
    .padding {
        width: 20px;
        padding: 0px;
    }
    tr:nth-child(2n) {
        background-color: var(--charcoal-dark1);
    }
    .delete-icon {
        color: var(--red);
        font-size: 20px;
        cursor: pointer;
    }

    @media (max-width: 400px) {
		table .date {
			display: none;
		}
        table .padding {
            width: 5px;
        }
	}
</style>