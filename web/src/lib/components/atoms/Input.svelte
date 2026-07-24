<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';

	type Props = Omit<HTMLInputAttributes, 'value'> & { value?: string };

	let { value = $bindable(''), class: className = '', ...rest }: Props = $props();
</script>

<input
	bind:value
	class="rounded border border-gray-300 px-3 py-2 outline-transparent transition-[outline] invalid:border-red-500 focus:outline-sky-500 {className}"
	{...rest}
/>
