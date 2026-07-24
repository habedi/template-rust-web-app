<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	type Props = HTMLButtonAttributes & {
		variant?: 'primary' | 'secondary' | 'danger';
		size?: 'sm' | 'md';
		children: Snippet;
	};

	let {
		variant = 'primary',
		size = 'md',
		class: className = '',
		children,
		...rest
	}: Props = $props();

	const variants = {
		primary: 'bg-sky-600 text-white hover:bg-sky-700',
		secondary: 'bg-gray-200 text-gray-800 hover:bg-gray-300',
		danger: 'bg-red-600 text-white hover:bg-red-700',
	};

	const sizes = {
		sm: 'px-3 py-1 text-sm',
		md: 'px-5 py-2',
	};
</script>

<button
	class="rounded transition disabled:cursor-not-allowed disabled:opacity-50 {variants[
		variant
	]} {sizes[size]} {className}"
	{...rest}
>
	{@render children()}
</button>
