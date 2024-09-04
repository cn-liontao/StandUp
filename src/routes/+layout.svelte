<script lang="ts">
	import { calendarState } from '$store/calendar';

	let isSystemDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches as boolean
	window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', ({ matches }) => {
		isSystemDarkMode = matches
	})
	$: isDarkMode = $calendarState.appSettings.theme === 'dark' || ($calendarState.appSettings.theme === 'system' && isSystemDarkMode)
</script>

<section class:dark={isDarkMode} class="h-screen overflow-hidden" >
	<main class="p-2 h-full dark:bg-neutral-900">
		<slot />
	</main>
</section>