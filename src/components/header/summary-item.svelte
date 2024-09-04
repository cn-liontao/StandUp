<script lang="ts">
	export let num: number | string
	export let text: string
</script>

<div class="flex flex-col text-neutral-6 dark:text-neutral-3">
	<div class="text-lg font-medium">{num}</div>
	<div class="text-neutral dark:text-neutral-4">{text}</div>
</div>