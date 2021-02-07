<script lang="ts">
	import { api } from '../api'
	import Groups from './Groups.svelte'
	import Login from './Login.svelte'

	let loading = true
	let current_user: string | null = null

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
</script>

{#if !loading}
	{#if current_user}
		<main>
			<header>
				<div>
					{#if current_user}
						Hello, {current_user}!
					{/if}
				</div>
				<button on:click={logout}>logout</button>
			</header>
			<Groups/>
		</main>
	{:else}
		<Login on_login={user => current_user = user}/>
	{/if}
{/if}

<style>
	header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
</style>