import {
	eachDayOfInterval,
	intlFormat,
	isSameWeek,
	lastDayOfMonth,
	lastDayOfWeek,
	startOfMonth,
	startOfWeek,
	subWeeks
} from 'date-fns/fp';

export const allDaysOfWeeksBefore = (startDate: Date, weekNum: number) => {
	const lastDay = lastDayOfWeek(startDate)
	const firstDay = startOfWeek(subWeeks(weekNum - 1)(startDate))
	const allDays = eachDayOfInterval({ start: firstDay, end: lastDay })

	return allDays.reduce<Date[][]>((acc, day) => {
		const currentWeek = acc[0]

		if (!currentWeek || currentWeek.length === 7) {
			return [[day], ...acc]
		} else {
			currentWeek.push(day)
		}

		return acc;
	}, [] as Date[][])
}

export const allDaysOfMonth = (startDate: Date) => {
	const lastDay = lastDayOfMonth(startDate)
	const firstDay = startOfMonth(startDate)

	return [eachDayOfInterval({ start: firstDay, end: lastDay })]
}

export const isFirstWeekOfMonth = (date: Date) => {
	const weekEnd = lastDayOfWeek(date)
	const monthStart = startOfMonth(weekEnd)

	return isSameWeek(monthStart)(weekEnd)
}

export const formatMonth = (date: Date) => {
	return intlFormat({ locale: 'zh-CN' })({ month: 'long' })(lastDayOfWeek(date))
}