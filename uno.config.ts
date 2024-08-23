import { defineConfig, presetIcons, presetUno } from 'unocss';
import extractorSvelte from '@unocss/extractor-svelte'

export default defineConfig({
	presets: [
		presetUno({}),
		presetIcons({ /* options */ }),
	],
	extractors: [
		extractorSvelte()
	],
	safelist: [
		...Array.from({ length: 9 }, (_, i) => `bg-green-${i + 1}00`),
	]
})