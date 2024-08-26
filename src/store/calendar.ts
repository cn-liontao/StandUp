import { derived, writable } from 'svelte/store';
import { produce } from 'immer'
import { addMonths, addWeeks, startOfDay, startOfMonth, startOfWeek, subMonths, subWeeks } from 'date-fns/fp';
import { allDaysOfMonth, allDaysOfWeeksBefore } from '$lib/calendar';
import { takeRightWhile } from 'lodash-es';
import { type DayRecord, dayRecords } from '$store/day-records';

interface CalendarState {
	calendarView: 'quarter' | 'month'
	anchor: Date
}

function createCalendarState() {
	const { update, subscribe } = writable<CalendarState>({
		calendarView: 'quarter',
		anchor: startOfDay(new Date()),
	})

	return {
		decrementAnchor: () => {
			update(produce(draft => {
				if (draft.calendarView === 'quarter') {
					draft.anchor = subWeeks(12)(draft.anchor)
				} else {
					draft.anchor = subMonths(1)(draft.anchor)
				}
			}))
		},
		incrementAnchor: () => {
			update(produce(draft => {
				if (draft.calendarView === 'quarter') {
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
		setDL: (dl: CalendarState['calendarView']) => { update(produce(draft => draft.calendarView = dl)) },
		subscribe
	}
}

export const calendarState = createCalendarState()

const getDaysInView = ($calendarState: CalendarState) => {
	const { calendarView, anchor } = $calendarState
	if (calendarView === 'quarter') {
		return allDaysOfWeeksBefore(anchor, 4 * 3)
	} else if (calendarView === 'month') {
		return allDaysOfMonth(anchor)
	}
	throw new Error('unknown calendar view');
}
export const daysInView = derived(calendarState, getDaysInView)

const getViewSlice = ([$calendarState, $dayRecords]: [CalendarState, DayRecord[]]) => {
	const { calendarView, anchor } = $calendarState

	let firstDay: number;
	if (calendarView === 'quarter') {
		firstDay = subWeeks(11)(startOfWeek(anchor)).getTime();
	} else {
		firstDay = startOfMonth(anchor).getTime();
	}

	return takeRightWhile($dayRecords, (day) => day.date > firstDay)
}
export const dayRecordsInView = derived([calendarState, dayRecords], getViewSlice)