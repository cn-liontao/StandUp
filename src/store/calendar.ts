import { derived, writable } from 'svelte/store';
import { produce } from 'immer'
import { addMonths, addWeeks, startOfDay, startOfMonth, startOfWeek, subMonths, subWeeks } from 'date-fns/fp';
import { allDaysOfMonth, allDaysOfWeeksBefore } from '$lib/calendar';
import { takeRightWhile } from 'lodash-es';
import { type DayRecord, dayRecords } from '$store/day-records';
import { invoke } from '@tauri-apps/api/tauri';
import { type Event, listen } from '@tauri-apps/api/event';

export interface AppSettings {
	theme: 'light' | 'dark'
	calendar_view: 'Quarter' | 'Month'
	hide_on_start: boolean
	start_with_system: boolean
}

function initAppSettings() {
	return {
		theme: 'light',
		calendar_view: 'Quarter',
		hide_on_start: false,
		start_with_system: false,
	} as AppSettings;
}

interface CalendarState {
	appSettings: AppSettings
	anchor: Date
}

function createCalendarState() {
	const { update, subscribe } = writable<CalendarState>({
		appSettings: initAppSettings(),
		anchor: startOfDay(new Date()),
	} as CalendarState)

	return {
		decrementAnchor: () => {
			update(produce(draft => {
				if (draft.appSettings.calendar_view === 'Quarter') {
					draft.anchor = subWeeks(12)(draft.anchor)
				} else {
					draft.anchor = subMonths(1)(draft.anchor)
				}
			}))
		},
		incrementAnchor: () => {
			update(produce(draft => {
				if (draft.appSettings.calendar_view === 'Quarter') {
					draft.anchor = addWeeks(12)(draft.anchor)
				} else {
					draft.anchor = addMonths(1)(draft.anchor)
				}
			}))
		},
		resetAnchor: () => {
			update(produce(draft => {
				draft.anchor = startOfDay(new Date())
			}))
		},
		updateAppSettings: (settings: Partial<AppSettings>) => {
			update(produce(draft => {
				draft.appSettings = {
					...draft.appSettings,
					...settings
				}
				invoke<undefined>('save_settings', {
					newSettings: draft.appSettings,
				}).then(() => {
					console.log('update')
				})
			}))
		},
		init: () => {
			invoke<AppSettings>('get_settings').then((settings) => {
				console.log('init', settings)
				update(produce(draft => { draft.appSettings = settings }))
			})
		},
		listen: () => {
			return listen('settings-update', (event: Event<AppSettings>) => {
				console.log('settings update:', event);
				update(produce(draft => { draft.appSettings = event.payload }))
			})
		},
		subscribe
	}
}

export const calendarState = createCalendarState()

const getDaysInView = ($calendarState: CalendarState) => {
	const { appSettings, anchor } = $calendarState
	const { calendar_view } = appSettings;

	if (calendar_view === 'Quarter') {
		return allDaysOfWeeksBefore(anchor, 4 * 3)
	} else if (calendar_view === 'Month') {
		return allDaysOfMonth(anchor)
	}
	throw new Error('unknown calendar view');
}
export const daysInView = derived(calendarState, getDaysInView)

const getViewSlice = ([$calendarState, $dayRecords]: [CalendarState, DayRecord[]]) => {
	const { appSettings, anchor } = $calendarState
	const { calendar_view } = appSettings;

	let firstDay: number;
	if (calendar_view === 'Quarter') {
		firstDay = subWeeks(11)(startOfWeek(anchor)).getTime();
	} else {
		firstDay = startOfMonth(anchor).getTime();
	}

	return takeRightWhile($dayRecords, (day) => day.date > firstDay)
}
export const dayRecordsInView = derived([calendarState, dayRecords], getViewSlice)

if (!import.meta.env.TEST) {
	calendarState.init()
}