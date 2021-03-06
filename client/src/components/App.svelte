<script lang="ts">
	import { api } from '../api'
	import AddNamed from './AddNamed.svelte'
	import Groups from './Groups.svelte'
	import Scorables from './Scorables.svelte'
	import Scores from './Scores.svelte'
	import Login from './Login.svelte'
	import Link from './Link.svelte'
import { onDestroy } from 'svelte';

	let loading = true
	let current_user: string | null = null
	let show_settings_modal = false

	type Page = {
		kind: "groups"
	} | {
		kind: "group"
		id: string
	} | {
		kind: "scores"
		id: string
		group_id: string
	}
	let page : Page = extract_page_from_hash()

	// Poll for the current user status. This will force us to a
	// login page when the session expires.
	get_current_user()
	let current_user_interval = setInterval(get_current_user, 1000)
	onDestroy(() => clearInterval(current_user_interval))

	//** A quick and dirty router:
	function extract_page_from_hash(): Page {
		const hash_parts = document.location.hash.replace(/^#\/?/, "").split("/")
		if (hash_parts[0] === "group" && hash_parts[1]) {
			return { kind: "group", id: hash_parts[1] }
		} else if (hash_parts[0] === "scores" && hash_parts[1]) {
			return { kind: "scores", group_id: hash_parts[1], id: hash_parts[2] }
		} else {
			return { kind: "groups" }
		}
	}
	function set_hash_from_page(page: Page) {
		if (page.kind === "group") {
			document.location.hash = `/group/${page.id}`
		} else if (page.kind === "scores") {
			document.location.hash = `/scores/${page.group_id}/${page.id}`
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

	function get_current_user() {
		api.current_user().then(user => {
			current_user = user.username
		}).finally(() => {
			loading = false
		})
	}

	function logout() {
		api.logout().then(_ => {
			current_user = null
		})
	}

	function settings() {
		show_settings_modal = true
	}

	function go_home() {
		change_page({ kind: "groups" })
	}
	function on_select_group(id: string) {
		change_page({ kind: "group", id })
	}
	function on_select_scorable(id: string, group_id: string) {
		change_page({ kind: "scores", id, group_id })
	}

	function change_password(password: string) {
		show_settings_modal = false
		api.upsert_user({ password })
	}

</script>

{#if !loading}
	{#if current_user}
		<main>
			<header>
				<h1 on:click={go_home}>Highscore</h1>
				<div class="greetings">
					<span>
						{#if current_user}
							Hello, {current_user}!
						{/if}
					</span>
					<span>
						<Link on_click={settings}>settings</Link>&nbsp;
						<Link on_click={logout}>logout</Link>
					</span>
				</div>
			</header>
			<div class="content">
				{#if page.kind === "groups"}
					<Groups {on_select_group}/>
				{:else if page.kind === "group"}
					<Scorables on_go_back={go_home} {on_select_scorable} group_id={page.id}/>
				{:else if page.kind === "scores"}
					<Scores on_go_back={on_select_group} current_user={current_user} group_id={page.group_id} scorable_id={page.id}/>
				{/if}
			</div>
		</main>
	{:else}
		<Login on_login={user => current_user = user}/>
	{/if}
{/if}

{#if show_settings_modal}
	<AddNamed
		title="Edit User"
		description="Set Password"
		type="password"
		on_cancel={() => { show_settings_modal = false }}
		on_try_add={change_password}
	/>
{/if}

<style>
	main {
		margin: 0;
		padding: 0;
		min-height: 100vh;
	}
	header {
		background-color: var(--charcoal-light2);
		padding: 15px;
		flex-shrink: 0;
	}

	header h1 {
		margin: 0;
		margin-bottom: 0px;
		margin-bottom: 10px;
		font-size: 40px;
		cursor: pointer;
		user-select: none;
	}
	header .greetings {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.content {
		margin-bottom: 2em;
	}
</style>