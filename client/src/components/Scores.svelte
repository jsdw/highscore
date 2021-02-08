<script lang="ts">
    import type { api as apiTypes } from '../api'
    import { api } from '../api'
    import AddNamed from './AddNamed.svelte'
    import Button from './Button.svelte'

    export let scorable_id: string

    let scores: apiTypes.ScoresOutput = []
    let name: string = ""

    get_details()

    async function get_details() {
        const res = await api.scores({ scorable_id })
        const scorable = await api.get_scorable({ id: scorable_id })
        scores = res
        name = scorable.name
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

</script>

<h1>{name}</h1>
<div class="container">
    <table>
        <tr>
            <th>Name</th>
            <th>Score</th>
            <th>Date</th>
        </tr>
        {#each scores as score (score.id) }
            <tr>
                <td>{score.username}</td>
                <td>{score.value}</td>
                <td>{pretty_print_iso_date(score.date)}</td>
            </tr>
        {/each}
    </table>
</div>

<style>
    h1 {
        text-align: center;
    }
    .container {
        margin: 1em;
        display: flex;
        justify-content: center;
    }
    table {
        width: 700px;
        max-width: calc(100% - 2em);
    }
    td, th {
        text-align: center;
    }
</style>