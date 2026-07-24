<script lang="ts">
	import type { Snippet } from 'svelte';
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import '../app.css';

	let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
	<title>{page.data.title ?? 'Home'} | Rust Web App</title>
</svelte:head>

<div class="min-h-screen bg-gray-50 text-gray-900">
	<header class="border-b border-gray-200 bg-white">
		<div class="mx-auto flex max-w-3xl items-center justify-between px-4 py-4">
			<a href={resolve('/')} class="text-lg font-bold text-sky-700">Rust Web App</a>
			<span class="text-sm text-gray-500">Template</span>
		</div>
	</header>
	<main class="mx-auto max-w-3xl px-4 py-8">
		{@render children()}
	</main>
</div>
