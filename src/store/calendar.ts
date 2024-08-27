import { derived, writable } from 'svelte/store';
import { produce } from 'immer'
import { addMonths, addWeeks, startOfDay, startOfMonth, startOfWeek, subMonths, subWeeks } from 'date-fns/fp';
import { allDaysOfMonth, allDaysOfWeeksBefore } from '$lib/calendar';
import { takeRightWhile } from 'lodash-es';
import { type DayRecord, dayRecords } from '$store/day-records';
import { invoke } from '@tauri-apps/api/tauri';
import { type Event, listen } from '@tauri-apps/api/event';

interface AppSettings {
	theme: 'light' | 'dark'
	calendarView: 'quarter' | 'month'
	hideOnStart: boolean
	startWithSystem: boolean
}

function initAppSettings() {
	return {
		theme: 'light',
		calendarView: 'quarter',
		hideOnStart: false,
		startWithSystem: false,
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
	})

	return {
		decrementAnchor: () => {
			update(produce(draft => {
				if (draft.appSettings.calendarView === 'quarter') {
					draft.anchor = subWeeks(12)(draft.anchor)
				} else {
					draft.anchor = subMonths(1)(draft.anchor)
				}
			}))
		},
		incrementAnchor: () => {
			update(produce(draft => {
				if (draft.appSettings.calendarView === 'quarter') {
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
		setDL: (dl: CalendarState['appSettings']['calendarView']) => {
			update(produce(draft => draft.appSettings.calendarView = dl))
		},
		init: () => {
			invoke<AppSettings>('get_settings').then((settings) => {
				update(produce(draft => draft.appSettings = settings))
			})
		},
		listen: () => {
			return listen('settings-update', (event: Event<AppSettings>) => {
				console.log('settings update:', event);
				update(produce(draft => draft.appSettings = event.payload))
			})
		},
		subscribe
	}
}

export const calendarState = createCalendarState()

const getDaysInView = ($calendarState: CalendarState) => {
	const { appSettings, anchor } = $calendarState
	const { calendarView } = appSettings;

	if (calendarView === 'quarter') {
		return allDaysOfWeeksBefore(anchor, 4 * 3)
	} else if (calendarView === 'month') {
		return allDaysOfMonth(anchor)
	}
	throw new Error('unknown calendar view');
}
export const daysInView = derived(calendarState, getDaysInView)

const getViewSlice = ([$calendarState, $dayRecords]: [CalendarState, DayRecord[]]) => {
	const { appSettings, anchor } = $calendarState
	const { calendarView } = appSettings;

	let firstDay: number;
	if (calendarView === 'quarter') {
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