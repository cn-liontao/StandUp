<script lang="ts">
	import { type AppSettings, calendarState } from '$store/calendar';

	const update = (settingKey: keyof AppSettings) => (e: Event) => {
		calendarState.updateAppSettings({ [settingKey]: (e.target as HTMLInputElement).value as AppSettings[typeof settingKey] });
	}
	const toggle = (settingKey: keyof AppSettings) => (e: Event) => {
		const checked = (e.target as HTMLInputElement).checked as boolean
		calendarState.updateAppSettings({ [settingKey]: checked });
	}
</script>

<form
	class="flex flex-col text-xs font-sans rounded-xl p-3 bg-gray-1 rounded-2xl"
>
	<label class="flex justify-between">
		主题色：
		<select
			name="theme"
			value={$calendarState.appSettings.theme}
			on:change={update('theme')}
		>
			<option class="text-sm" value="light">明亮</option>
			<option class="text-sm" value="dark">暗黑</option>
		</select>
	</label>
	<label class="flex justify-between">
		当前视图：
		<select
			name="calendarView"
			value={$calendarState.appSettings.calendar_view}
			on:change={update('calendar_view')}
		>
			<option value="Quarter">季度</option>
			<option value="Month">月度</option>
		</select>
	</label>
	<label class="flex justify-between">
		启动时隐藏：
		<input
			type="checkbox"
			checked={$calendarState.appSettings.hide_on_start}
			on:change={toggle('hide_on_start')}
		/>
	</label>
	<label class="flex justify-between">
		跟随系统启动：
		<input
			type="checkbox"
			checked={$calendarState.appSettings.start_with_system}
			on:change={toggle('start_with_system')}
		/>
	</label>
</form>

