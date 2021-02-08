<script lang="ts">
	import { api } from '../api'
	import Groups from './Groups.svelte'
	import Scorables from './Scorables.svelte'
	import Scores from './Scores.svelte'
	import Login from './Login.svelte'
	import Link from './Link.svelte'

	let loading = true
	let current_user: string | null = null

	type Page = {
		kind: "groups"
	} | {
		kind: "group"
		id: string
	} | {
		kind: "scores"
		id: string
	}
	let page : Page = extract_page_from_hash()

	//** A quick and dirty router:
	function extract_page_from_hash(): Page {
		const hash_parts = document.location.hash.replace(/^#\/?/, "").split("/")
		if (hash_parts[0] === "group" && hash_parts[1]) {
			return { kind: "group", id: hash_parts[1] }
		} else if (hash_parts[0] === "scores" && hash_parts[1]) {
			return { kind: "scores", id: hash_parts[1] }
		} else {
			return { kind: "groups" }
		}
	}
	function set_hash_from_page(page: Page) {
		if (page.kind === "group") {
			document.location.hash = `/group/${page.id}`
		} else if (page.kind === "scores") {
			document.location.hash = `/scores/${page.id}`
		} else {
			document.location.hash = ""
		}
	}
	function change_page(dest: Page) {
		page = dest
		set_hash_from_page(page)
	}
	window.addEventListener("hashchange", () => {
		page = extract_page_from_hash()
	})
	//**

	api.current_user().then(user => {
		current_user = user.username
	}).finally(() => {
		loading = false
	})

	function logout() {
		api.logout().then(_ => {
			current_user = null
		})
	}

	function on_select_group(id: string) {
		change_page({
			kind: "group",
			id
		})
	}
	function on_select_scorable(id: string) {
		change_page({
			kind: "scores",
			id
		})
	}

</script>

{#if !loading}
	{#if current_user}
		<main>
			<header>
				<h1>Highscore</h1>
				<div class="greetings">
					<span>
						{#if current_user}
							Hello, {current_user}!
						{/if}
					</span>
					<Link on_click={logout}>logout</Link>
				</div>
			</header>
			{#if page.kind === "groups"}
				<Groups {on_select_group}/>
			{:else if page.kind === "group"}
				<Scorables {on_select_scorable} group_id={page.id}/>
			{:else if page.kind === "scores"}
				<Scores scorable_id={page.id}/>
			{/if}
		</main>
	{:else}
		<Login on_login={user => current_user = user}/>
	{/if}
{/if}

<style>
	header {
		background-color: var(--charcoal-light2);
		padding: 15px;
	}
	h1 {
		margin: 0;
		margin-bottom: 0px;
		margin-bottom: 10px;
		font-size: 40px;
	}
	.greetings {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
</style>