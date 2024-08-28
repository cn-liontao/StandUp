<script context="module" lang="ts">
	import { getContext, setContext } from 'svelte';
	import { type Writable, writable } from 'svelte/store';

	export const provideTooltip = () => {
		const tooltipContent = writable<string>('')

		setContext('tooltip', tooltipContent)

		return tooltipContent
	}

	export const useTooltip: () => Writable<string> = () => {
		return getContext('tooltip');
	}
</script>

<script lang="ts">
	export let content: string = ""
</script>

{#if content}
	<div class="px-1.8 py-1 opacity-70 text-white bg-gray-800 rounded-md fixed left-1.5 bottom-1.5">
		<div class="font-sans text-sm">{content}</div>
	</div>
{/if}