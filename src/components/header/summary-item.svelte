<script lang="ts">
	export let num: number | string
	export let text: string
</script>

<div class="flex flex-col text-stone-600">
	<div class="text-lg font-medium">{num}</div>
	<div class="text-stone">{text}</div>
</div>