<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import Button from '$lib/components/atoms/Button.svelte';
	import Error from '$lib/components/Error.svelte';
	import Icon from '$lib/components/atoms/Icon.svelte';
	import Input from '$lib/components/atoms/Input.svelte';
	import { createItem, deleteItem } from '$lib/services/api';
	import { formatTimestamp } from '$lib/services/utils';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let name = $state('');
	let description = $state('');
	let pending = $state(false);
	let errorMessage = $state<string | null>(null);

	const submit = async (event: SubmitEvent) => {
		event.preventDefault();
		pending = true;
		errorMessage = null;

		try {
			await createItem({ name, description: description || null });
			name = '';
			description = '';
			await invalidateAll();
		} catch {
			errorMessage = 'Could not create the item.';
		} finally {
			pending = false;
		}
	};

	const remove = async (id: string) => {
		errorMessage = null;

		try {
			await deleteItem(id);
			await invalidateAll();
		} catch {
			errorMessage = 'Could not delete the item.';
		}
	};
</script>

<h1 class="text-3xl font-bold">Items</h1>
<p class="mt-1 text-sm text-gray-500">
	An example resource served by the backend at <code>/api/v1/items</code>.
</p>

<form class="mt-6 flex flex-col gap-2 sm:flex-row" onsubmit={submit}>
	<Input bind:value={name} placeholder="Name" required maxlength={128} class="sm:flex-1" />
	<Input
		bind:value={description}
		placeholder="Description (optional)"
		maxlength={1024}
		class="sm:flex-1"
	/>
	<Button type="submit" disabled={pending || name.trim().length === 0}>
		<Icon type="plus-circle" class="mr-1 inline-block h-5 w-5 align-text-bottom" />
		Add
	</Button>
</form>

{#if errorMessage}
	<Error message={errorMessage} />
{/if}

{#if data.items.length === 0}
	<p class="mt-8 text-center text-gray-500">No items yet. Add the first one above.</p>
{:else}
	<ul class="mt-6 divide-y divide-gray-200 rounded border border-gray-200 bg-white">
		{#each data.items as item (item.id)}
			<li class="flex items-center justify-between gap-4 px-4 py-3">
				<div class="min-w-0">
					<p class="truncate font-medium">{item.name}</p>
					{#if item.description}
						<p class="truncate text-sm text-gray-500">{item.description}</p>
					{/if}
					<p class="text-xs text-gray-400">Created {formatTimestamp(item.created_at)}</p>
				</div>
				<Button
					variant="danger"
					size="sm"
					onclick={() => remove(item.id)}
					aria-label="Delete {item.name}"
				>
					<Icon type="trash" class="h-4 w-4" />
				</Button>
			</li>
		{/each}
	</ul>
{/if}
